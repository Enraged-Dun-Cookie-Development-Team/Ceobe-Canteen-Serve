use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseBackend, DbErr, EntityTrait,
    QueryFilter, QuerySelect, Statement,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;

use super::{FetcherPlatformConfigSqlOperate, OperateResult, PlatformCounts};
use crate::fetcher::platform_config::models::model_platform_config;

impl FetcherPlatformConfigSqlOperate {
    // 查询是否存在type_id的平台
    pub async fn is_platform_exist(
        db: &impl ConnectionTrait, type_id: &str,
    ) -> OperateResult<bool> {
        let resp = model_platform_config::Entity::find()
            .filter(model_platform_config::Column::TypeId.eq(type_id))
            .select_only()
            .column_as(model_platform_config::Column::Id.count(), "count")
            .into_model::<PlatformCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }

    // 查询是否存在type_id的平台
    pub async fn is_platform_exist_with_raw_db<'db, D>(
        db: &'db D, type_id: &str,
    ) -> OperateResult<bool>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        let resp = model_platform_config::Entity::find()
            .filter(model_platform_config::Column::TypeId.eq(type_id))
            .select_only()
            .column_as(model_platform_config::Column::Id.count(), "count")
            .into_model::<PlatformCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }

    // 查询id的平台下时候有数据源
    pub async fn has_datasource_with_id(
        db: &impl ConnectionTrait, pid: i32,
    ) -> OperateResult<bool> {
        let sql = String::from(&format!(
            r#"
            SELECT count(d.id) as count FROM fetcher_platform_config p left JOIN fetcher_datasource_config d ON p.type_id = d.platform where p.id = {pid};
        "#
        ));

        let resp = model_platform_config::Entity::find()
            .from_raw_sql(Statement::from_string(DatabaseBackend::MySql, sql))
            .into_model::<PlatformCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }
}
