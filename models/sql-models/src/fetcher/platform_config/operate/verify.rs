use futures::Future;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use sql_connection::ext_traits::{
    check_all_exist::QueryAllExist, select_count::QueryCountByColumn,
};

use super::{FetcherPlatformConfigSqlOperate, OperateResult};
use crate::fetcher::{
    datasource_config::models::model_datasource_config,
    platform_config::models::model_platform_config::{
        self,
        Column::{Id, TypeId},
        Entity,
    },
};

impl FetcherPlatformConfigSqlOperate {
    #[allow(clippy::manual_async_fn)]
    pub fn all_exist_by_type_ids<'db, I>(
        db: &'db impl ConnectionTrait, type_ids: I,
    ) -> impl Future<Output = OperateResult<bool>> + Send + 'db
    where
        <I as IntoIterator>::IntoIter: std::marker::Send,
        I: IntoIterator<Item = &'db str> + Send + 'db,
    {
        async {
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
    }

    /// 查询是否存在type_id的平台
    pub async fn exist_by_type_id(
        db: &impl ConnectionTrait, type_id: &str,
    ) -> OperateResult<bool> {
        let count = Entity::find()
            .filter(TypeId.eq(type_id))
            .count_non_zero_by_column(Id)
            .one(db)
            .await?
            .unwrap()
            .take();

        Ok(count)
    }

    /// 查询id的平台下时候有数据源
    pub async fn has_datasource_by_id(
        db: &impl ConnectionTrait, platform_id: i32,
    ) -> OperateResult<bool> {
        let resp = Entity::find()
            .left_join(model_datasource_config::Entity)
            .filter(model_platform_config::Column::Id.eq(platform_id))
            .count_non_zero_by_column(model_datasource_config::Column::Id)
            .one(db)
            .await?
            .unwrap()
            .take();

        Ok(resp)
    }
}
#[cfg(test)]
mod test {
    use super::Entity;
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

    use sea_orm::{
        ColumnTrait, DatabaseBackend, EntityTrait, QueryFilter, QueryTrait,
        Select,
    };
    use sea_query::{Alias, Expr, MysqlQueryBuilder, Query, SelectStatement};
    use sql_connection::ext_traits::{
        check_all_exist::QueryAllExist,
        select_count::{ColumnExpr, QueryCountByColumn},
    };

    use crate::fetcher::{
        datasource_config::models::model_datasource_config,
        platform_config::models::model_platform_config,
    };
    #[test]
    fn test_gen_sql() {
        let query = super::Entity::find()
            .left_join(model_datasource_config::Entity)
            .filter(model_platform_config::Column::Id.eq(11))
            .select_count_by_colum(model_datasource_config::Column::Id)
            .build(DatabaseBackend::MySql);

        let query2 = gen_query_verify_has_datasource_with_id(11);
        let sql_str = query.to_string().to_lowercase();
        let sql_str2 = query2.to_string(MysqlQueryBuilder).to_lowercase();

        println!("{sql_str}");
        println!();
        println!("{sql_str2}");

        assert_eq!(sql_str, sql_str2)
    }
    #[test]
    fn test_all_exist() {
        let query = Select::gen_statement(
            model_platform_config::Entity,
            model_platform_config::Column::TypeId,
            "a",
            ["b", "v", "d"],
            &DatabaseBackend::MySql,
        )
        .to_string();

        println!("{query}")
    }
    #[test]
    fn test_count_start() {
        let query = super::Entity::find()
            .select_count_by_colum(ColumnExpr::asterisk())
            .build(DatabaseBackend::MySql);

        println!("{}", query.to_string())
    }
}
