use scheduler_notifier::SchedulerNotifier;
use sql_models::{
    fetcher::{
        config::operate::FetcherConfigSqlOperate,
        datasource_config::{
            checkers::FetcherDatasourceConfig,
            models::model_datasource_config::DatasourcePlatform,
            operate::FetcherDatasourceConfigSqlOperate,
        },
        platform_config::operate::FetcherPlatformConfigSqlOperate,
    },
    sql_connection::{
        database_traits::get_connect::{
            GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
        },
        sea_orm::{ConnectionTrait, DbErr},
    },
};

use crate::{
    error::{LogicError, LogicResult},
    implements::FetcherConfigLogic,
    utils::TrueOrError,
};

impl FetcherConfigLogic {
    /// 新建数据源配置
    pub async fn create_datasource_config<'db, D>(
        db: &'db D, datasource_config: FetcherDatasourceConfig,
    ) -> LogicResult<()>
    where
        D: GetDatabaseConnect + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect();
        // 验证平台存在
        FetcherPlatformConfigSqlOperate::exist_by_type_id(
            db,
            &datasource_config.platform,
        )
        .await?
        .true_or_with(|| LogicError::PlatformNotFound)?;
        // 创建数据源
        FetcherDatasourceConfigSqlOperate::create(db, datasource_config)
            .await?;
        Ok(())
    }

    /// 删除一个数据源
    pub async fn delete_datasource_by_id<'db, D>(
        notifier: &SchedulerNotifier, db: &'db D, id: i32,
    ) -> LogicResult<()>
    where
        D: GetDatabaseTransaction<Error = DbErr> + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        // 开事务
        let ctx = db.get_transaction().await?;

        // 删除蹲饼器配置中的所有有datasource_id的配置
        FetcherConfigSqlOperate::delete_by_datasource_id(&ctx, id).await?;

        let DatasourcePlatform { platform } =
            FetcherDatasourceConfigSqlOperate
                .find_platform_by_id(&ctx, id)
                .await?;

        // 删除数据源
        FetcherDatasourceConfigSqlOperate::delete_one(&ctx, id).await?;

        // 提交事务
        ctx.submit().await?;
        notifier.notify_platform_update(platform).await;

        Ok(())
    }
}
