
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{instrument, info};

use crate::fetcher::datasource_config::models::model_datasource_config;

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};

impl FetcherDatasourceConfigSqlOperate {
    #[instrument(skip(db), ret)]
    // 删除一个平台
    pub async fn delete_one_datasource_config<'db, D>(
        db: &'db D, did: i32,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(datasource.id = did);
        let db = db.get_connect()?;

        model_datasource_config::Entity::delete_by_id(did).exec(db).await?;

        Ok(())
    }
}
