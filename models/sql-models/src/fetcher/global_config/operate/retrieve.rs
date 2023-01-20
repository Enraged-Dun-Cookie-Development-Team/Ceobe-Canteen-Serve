use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::instrument;

use super::{FetcherGlobalConfigSqlOperate, OperateResult};
use crate::fetcher::global_config::models::model_global_config::{
    self, Entity,
};

impl FetcherGlobalConfigSqlOperate {
    #[instrument(ret, skip_all)]
    pub async fn get_all<'db, D>(
        db: &'db D,
    ) -> OperateResult<Vec<model_global_config::Model>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        // 查询所有的键值对
        Ok(Entity::find().all(db).await?)
    }
}
