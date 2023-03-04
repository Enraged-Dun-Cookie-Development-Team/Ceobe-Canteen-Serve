use bool_or::TrueOrError;
use fetcher::{config::{ToConfig, ConfigOperate}, datasource_config::{ToDatasource, DatasourceOperate}, platform_config::PlatformOperate};
use scheduler_notifier::SchedulerNotifier;
use sql_models::{
    fetcher::{
        datasource_config::{
            checkers::FetcherDatasourceConfig,
            models::model_datasource_config::DatasourcePlatform,
        },
    },
    sql_connection::{
        database_traits::get_connect::{
            GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
        },
        sea_orm::{ConnectionTrait, DbErr}, SqlDatabaseOperate,
    },
};
use abstract_database::fetcher::ToFetcher;

use crate::{
    error::{LogicError, LogicResult},
    implements::FetcherConfigLogic,
};

impl FetcherConfigLogic {
    /// 新建数据源配置
    pub async fn create_datasource_config(
        db: SqlDatabaseOperate, datasource_config: FetcherDatasourceConfig,
    ) -> LogicResult<()> {
        let db = db.get_connect();
        // 验证平台存在
        PlatformOperate::exist_by_type_id(db, &datasource_config.platform)
            .await?
            .true_or_with(|| LogicError::PlatformNotFound)?;
        // 创建数据源
        DatasourceOperate::create(db, datasource_config).await?;
        Ok(())
    }

    /// 删除一个数据源
    pub async fn delete_datasource_by_id (
        notifier: &SchedulerNotifier, db: SqlDatabaseOperate, id: i32,
    ) -> LogicResult<()> {
        // 开事务
        let ctx = db.get_transaction().await?;

        // 删除蹲饼器配置中的所有有datasource_id的配置
        ConfigOperate::delete_by_datasource_id(&ctx, id).await?;

        let DatasourcePlatform { platform } =
             DatasourceOperate::find_platform_by_id(&ctx, id).await?;

        // 删除数据源
        DatasourceOperate::delete_one(&ctx, id).await?;

        // 提交事务
        ctx.submit().await?;
        notifier.notify_platform_update(platform).await;

        Ok(())
    }
}
