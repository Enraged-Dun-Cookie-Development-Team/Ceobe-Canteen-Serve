use sea_orm::{ConnectionTrait, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::instrument;

use super::{GlobalOperate, OperateResult};
use crate::fetcher::global_config::models::model_global_config::{
    self, Entity,
};

impl<'c, C> GlobalOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(ret, skip_all)]
    pub async fn get_all(
        &self,
    ) -> OperateResult<Vec<model_global_config::Model>> {
        let db = self.get_connect();
        // 查询所有的键值对
        Ok(Entity::find().all(db).await?)
    }
}
