use sea_orm::{ConnectionTrait, EntityTrait};
use sql_connection::database_traits::database_operates::NoConnect;
use tracing::{info, instrument};

use super::{Datasource, OperateResult};
use crate::fetcher::datasource_config::models::model_datasource_config::Entity;

impl Datasource<'_, NoConnect> {
    #[instrument(skip(db), ret)]
    /// 删除一个平台
    pub async fn delete_one(
        db: &impl ConnectionTrait, did: i32,
    ) -> OperateResult<()> {
        info!(datasource.id = did);
        Entity::delete_by_id(did).exec(db).await?;

        Ok(())
    }
}
