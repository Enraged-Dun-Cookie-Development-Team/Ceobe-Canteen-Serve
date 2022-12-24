use sea_orm::{Statement, ConnectionTrait, DbErr, EntityTrait, DbBackend, DatabaseBackend};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::instrument;

use crate::fetcher::datasource_config::models::model_datasource_config;

use super::{FetcherDatasourceConfigSqlOperate, OperateResult, DatasourceCounts};

impl FetcherDatasourceConfigSqlOperate {
    // 验证id数组是否都存在
    #[instrument(ret, skip(db))]
    pub async fn has_all_datasource_ids<'db, D>(
        db: &'db D, ids: Vec<i32>,
    ) -> OperateResult<bool>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let mut sql = String::from("select count(B.id) from (");
        for id in ids {
            sql.push_str(&format!(r#" select {id} as id from dual union"#));
        }
        sql = sql.trim_end_matches("union").to_string();
        sql.push_str(&format!(r#") B left join fetcher_datasource_config on fetcher_datasource_config.id = B.id where fetcher_datasource_config.id is null;"#));

        let db = db.get_connect()?;
        let resp = model_datasource_config::Entity::find()
            .from_raw_sql(Statement::from_string(
                DatabaseBackend::MySql,
                sql
            ))
            .into_model::<DatasourceCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count == 0)
    }
}
