use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, IntoActiveModel};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};
use tracing_unwrap::ResultExt;

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};
use crate::fetcher::datasource_config::{
    checkers::datasource_config_data::FetcherDatasourceConfig,
    models::model_datasource_config::ActiveModel,
};

impl FetcherDatasourceConfigSqlOperate {
    // 更新数据配置到数据库
    #[instrument(ret, skip(db))]
    pub async fn update_platform_config<'db, D>(
        db: &'db D, config: FetcherDatasourceConfig,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            config.id = config.id,
            datasource.name = config.nickname,
            datasource.avatar = config.avatar.to_string(),
            datasouce.config = serde_json::to_string(&config.config)
                .expect_or_log("config为非法json格式"),
        );

        let db = db.get_connect()?;
        let platform_config_active =
            ActiveModel::datasource_config_into_active_model(config);
        platform_config_active
            .into_active_model()
            .update(db)
            .await?;

        Ok(())
    }
}
