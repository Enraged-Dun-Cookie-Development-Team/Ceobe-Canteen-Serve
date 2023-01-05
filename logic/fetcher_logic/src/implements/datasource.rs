use sql_models::{
    fetcher::{
        config::operate::FetcherConfigSqlOperate,
        datasource_config::{
            checkers::datasource_config_data::FetcherDatasourceConfig,
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
};

impl FetcherConfigLogic {
    /// 新建数据源配置
    pub async fn create_datasource_config<'db, D>(
        db: &'db D, datasource_config: FetcherDatasourceConfig,
    ) -> LogicResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        // 验证平台存在
        if FetcherPlatformConfigSqlOperate::is_platform_exist_with_raw_db(
            db,
            &datasource_config.platform,
        )
        .await?
        {
            // 创建数据源
            FetcherDatasourceConfigSqlOperate::create_database_config(
                db,
                datasource_config,
            )
            .await?;
        }
        else {
            return Err(LogicError::NoPlatform);
        }
        Ok(())
    }

    /// 删除一个数据源
    pub async fn delete_datasource_by_id<'db, D>(
        db: &'db D, id: i32,
    ) -> LogicResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + GetDatabaseTransaction + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        // 开事务
        let ctx = db.get_transaction().await?;

        // 删除蹲饼器配置中的所有有datasource_id的配置
        FetcherConfigSqlOperate::delete_fetcher_configs_by_datasource_id(
            &ctx, id,
        )
        .await?;
        // 删除数据源
        FetcherDatasourceConfigSqlOperate::delete_one_datasource_config(
            &ctx, id,
        )
        .await?;

        // 提交事务
        ctx.submit().await?;

        // TODO：告诉调度器哪个平台更新了

        Ok(())
    }
}