use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseBackend, DbErr, EntityTrait,
    QueryFilter, QuerySelect, Statement,
};
use sea_query::{Alias, Expr, Query, SelectStatement};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;

use super::{FetcherPlatformConfigSqlOperate, OperateResult, PlatformCounts};
use crate::fetcher::{
    datasource_config::{
        checkers::datasource_config_data::FetcherDatasourceConfig,
        models::model_datasource_config,
    },
    platform_config::models::model_platform_config,
};

impl FetcherPlatformConfigSqlOperate {
    /// 查询是否存在type_id的平台
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

    /// 查询是否存在type_id的平台
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

    /// 查询id的平台下时候有数据源
    pub async fn has_datasource_with_id(
        db: &impl ConnectionTrait, pid: i32,
    ) -> OperateResult<bool> {
        let query = gen_query_verfy_has_datasource_with_id(pid);

        let resp = model_platform_config::Entity::find()
            .from_raw_sql(sea_orm::StatementBuilder::build(
                &query,
                &db.get_database_backend(),
            ))
            .into_model::<PlatformCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }
}

/// 生成Statement 为 检查给定的id的平台下时候有数据源
fn gen_query_verfy_has_datasource_with_id(pid: i32) -> SelectStatement {
    let mut query = Query::select();

    // select count (fetcher_datasource_config.id) as count
    query.expr_as(
        Expr::tbl(
            model_datasource_config::Entity,
            model_datasource_config::Column::Id,
        )
        .count(),
        Alias::new("count"),
    );

    // from ()
    query.from(model_platform_config::Entity);

    // join
    query.left_join(
        model_datasource_config::Entity,
        Expr::tbl(
            model_platform_config::Entity,
            model_platform_config::Column::TypeId,
        )
        .equals(
            model_datasource_config::Entity,
            model_datasource_config::Column::Platform,
        ),
    );

    // where
    query.and_where(
        Expr::tbl(
            model_platform_config::Entity,
            model_platform_config::Column::Id,
        )
        .eq(pid),
    );

    query
}

#[cfg(test)]
mod test {
    use sea_query::MySqlQueryBuilder;

    use super::gen_query_verfy_has_datasource_with_id;
    #[test]
    fn test_gen_sql() {
        let query = gen_query_verfy_has_datasource_with_id(8);
        let sql_str = query.to_string(MySqlQueryBuilder::default());

        println!("{sql_str}")
    }
}
