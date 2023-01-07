use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use sea_query::{Alias, Expr, Query, SelectStatement};
use sql_connection::ext_traits::{
    check_all_exist::AllExist, select_count::SelectCount,
};

use super::{FetcherPlatformConfigSqlOperate, OperateResult, PlatformCounts};
use crate::fetcher::{
    datasource_config::models::model_datasource_config,
    platform_config::models::model_platform_config::{
        self,
        Column::{Id, TypeId},
        Entity,
    },
};

impl FetcherPlatformConfigSqlOperate {
    pub async fn all_exist_by_type_ids(
        db: &impl ConnectionTrait, type_ids: impl IntoIterator<Item = &str>,
    ) -> OperateResult<bool> {
        let mut iter = type_ids.into_iter();
        let Some(first) = iter.next()else{
            return Ok(true);
        };

        let count = Entity::find()
            .all_exist(
                Entity,
                TypeId,
                first,
                iter,
                &db.get_database_backend(),
            )
            .one(db)
            .await?
            .unwrap()
            .take();

        Ok(count)
    }

    /// 查询是否存在type_id的平台
    pub async fn exist_by_type_id(
        db: &impl ConnectionTrait, type_id: &str,
    ) -> OperateResult<bool> {
        let count = Entity::find()
            .filter(TypeId.eq(type_id))
            .select_count(Id, db)
            .await?
            .is_empty();

        Ok(!count)
    }

    /// 查询id的平台下时候有数据源
    pub async fn has_datasource_with_id(
        db: &impl ConnectionTrait, pid: i32,
    ) -> OperateResult<bool> {
        let query = gen_query_verify_has_datasource_with_id(pid);

        let resp = Entity::find()
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
fn gen_query_verify_has_datasource_with_id(pid: i32) -> SelectStatement {
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
    query.from(Entity);

    // join
    query.left_join(
        model_datasource_config::Entity,
        Expr::tbl(Entity, model_platform_config::Column::TypeId).equals(
            model_datasource_config::Entity,
            model_datasource_config::Column::Platform,
        ),
    );

    // where
    query.and_where(
        Expr::tbl(Entity, model_platform_config::Column::Id).eq(pid),
    );

    query
}

#[cfg(test)]
mod test {
    use sea_orm::Select;
    use sea_query::MySqlQueryBuilder;
    use sql_connection::ext_traits::check_all_exist::AllExist;

    use crate::fetcher::platform_config::models::model_platform_config;

    use super::gen_query_verify_has_datasource_with_id;
    #[test]
    fn test_gen_sql() {
        let query = gen_query_verify_has_datasource_with_id(8);
        let sql_str = query.to_string(MySqlQueryBuilder::default());

        println!("{sql_str}")
    }
    #[test]
    fn test_all_exist() {
        let query = Select::gen_statement(
            model_platform_config::Entity,
            model_platform_config::Column::TypeId,
            "a",
            ["b", "v", "d"],
            &sea_orm::DatabaseBackend::MySql,
        )
        .to_string();

        println!("{query}")
    }
}
