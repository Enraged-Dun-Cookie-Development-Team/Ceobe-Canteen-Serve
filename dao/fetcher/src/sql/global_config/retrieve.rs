use db_ops_prelude::{sea_orm::{ConnectionTrait, EntityTrait}, get_connect::GetDatabaseConnect, sql_models::fetcher::global_config::models::model_global_config::{self, Entity}};
use tracing::instrument;

use super::{GlobalOperate, OperateResult};

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
