use bool_or::TrueOrError;
use scheduler_notifier::SchedulerNotifier;
use sql_models::{
    fetcher::{
        config::operate::Config,
        datasource_config::{
            checkers::FetcherDatasourceConfig,
            models::model_datasource_config::DatasourcePlatform,
            operate::Datasource,
        },
        platform_config::operate::Platform,
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
};

impl FetcherConfigLogic {
    /// 新建数据源配置
    pub async fn create_datasource_config<D>(
        db: &D, datasource_config: FetcherDatasourceConfig,
    ) -> LogicResult<()>
    where
        D: GetDatabaseConnect + 'static,
        D::Connect: ConnectionTrait,
    {
        let db = db.get_connect();
        // 验证平台存在
        Platform::exist_by_type_id(db, &datasource_config.platform)
            .await?
            .true_or_with(|| LogicError::PlatformNotFound)?;
        // 创建数据源
        Datasource::create(db, datasource_config).await?;
        Ok(())
    }

    /// 删除一个数据源
    pub async fn delete_datasource_by_id<'t, 'db, D>(
        notifier: &SchedulerNotifier, db: &'db D, id: i32,
    ) -> LogicResult<()>
    where
        D: GetDatabaseTransaction<Error = DbErr> + 'db,
        D::Transaction<'t>: ConnectionTrait,
        'db: 't,
    {
        // 开事务
        let ctx = db.get_transaction().await?;

        // 删除蹲饼器配置中的所有有datasource_id的配置
        Config::delete_by_datasource_id(&ctx, id).await?;

        let DatasourcePlatform { platform } =
            Datasource::find_platform_by_id(&ctx, id).await?;

        // 删除数据源
        Datasource::delete_one(&ctx, id).await?;

        // 提交事务
        ctx.submit().await?;
        notifier.notify_platform_update(platform).await;

        Ok(())
    }
}
