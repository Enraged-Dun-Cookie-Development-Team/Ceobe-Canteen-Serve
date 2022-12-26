
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{instrument, info};

use crate::fetcher::datasource_config::models::model_datasource_config;

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};

impl FetcherDatasourceConfigSqlOperate {
    #[instrument(skip(db), ret)]
    // 删除一个平台
    pub async fn delete_one_datasource_config(
        db: &impl ConnectionTrait, did: i32,
    ) -> OperateResult<()> {
        info!(datasource.id = did);
        model_datasource_config::Entity::delete_by_id(did).exec(db).await?;

        Ok(())
    }
}
