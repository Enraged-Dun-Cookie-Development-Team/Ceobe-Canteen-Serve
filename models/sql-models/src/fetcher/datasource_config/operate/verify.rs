use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
};

use sea_orm::{
    sea_query::{Cond, Expr, Query},
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
    QuerySelect,
};
use sea_query::{Alias, SelectStatement, UnionType};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::instrument;

use super::{
    DatasourceCounts, FetcherDatasourceConfigSqlOperate, OperateResult,
};
use crate::fetcher::datasource_config::{
    models::model_datasource_config::{Column, Entity},
    operate::PlatformDatasource,
};

impl FetcherDatasourceConfigSqlOperate {
    // 验证id数组是否都存在
    #[instrument(ret, skip(db))]
    pub async fn has_all_datasource_ids<'db, D, T>(
        db: &'db D, ids: T,
    ) -> OperateResult<bool>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
        T: IntoIterator<Item = i32> + Debug,
        T::Item: Display,
    {
        let mut ids = ids.into_iter();
        let Some(first) = ids.next() else{
            return Ok(true);
        };
        let query = gen_query_verfy_all_datasource_id_exist(first, ids);
        // let mut sql = String::from("select count(B.id) from (");
        // for id in ids.into_iter() {
        //     sql.push_str(&format!(r#" select {id} as id from dual union"#));
        // }
        // sql = sql.trim_end_matches("union").to_string();
        // sql.push_str(
        //     ") B left join fetcher_datasource_config on \
        //      fetcher_datasource_config.id = B.id where \
        //      fetcher_datasource_config.id is null;",
        // );

        let db = db.get_connect()?;
        let resp = Entity::find()
            .from_raw_sql(sea_orm::StatementBuilder::build(
                &query,
                &db.get_database_backend(),
            ))
            .into_model::<DatasourceCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count == 0)
    }

    // 验证平台下是否还有数据源
    #[instrument(ret, skip(db))]
    pub async fn has_datasource_from_platforms<'db, D>(
        db: &'db D, platforms: Vec<String>,
    ) -> OperateResult<BTreeSet<String>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let mut condition = Cond::any();
        for platform in platforms {
            condition = condition.add(Column::Platform.eq(platform))
        }

        let db = db.get_connect()?;
        let resp = Entity::find()
            .select_only()
            .column(Column::Platform)
            .filter(condition)
            .group_by(Column::Platform)
            .into_model::<PlatformDatasource>()
            .all(db)
            .await?;

        let mut exist_map = BTreeSet::new();
        for platform_datasource in resp {
            exist_map.insert(platform_datasource.platform);
        }

        Ok(exist_map)
    }
}

/// 生成Statement 为 检查给定的 datasourece Id 是否均存在.
fn gen_query_verfy_all_datasource_id_exist(
    first: i32, ids: impl IntoIterator<Item = i32>,
) -> SelectStatement {
    // 临时表信息
    #[derive(sea_query::Iden)]
    enum TempTable {
        Table,
        Id,
    }

    let mut query = Query::select();

    // select count (B.id)
    query.expr(Expr::tbl(TempTable::Table, TempTable::Id).count());

    // from ()
    query.from_subquery(
        {
            // 取出第一个做base
            let mut query = Query::select();
            query.expr_as(Expr::val(first), TempTable::Id);
            // 剩下的union上, 使用去重union, select from DUAL
            query.unions(ids.into_iter().map(|idx| {
                (UnionType::Distinct, {
                    let mut union_query = Query::select();
                    union_query.expr_as(Expr::val(idx), TempTable::Id);
                    union_query
                })
            }));

            query
        },
        TempTable::Table,
    );

    // join
    query.left_join(
        Entity,
        Expr::tbl(Entity, Column::Id).equals(TempTable::Table, TempTable::Id),
    );
    // where
    query.and_where(Expr::tbl(Entity, Column::Id).is_null());

    query
}

#[cfg(test)]
mod test {
    use sea_query::MySqlQueryBuilder;

    use super::gen_query_verfy_all_datasource_id_exist;
    #[test]
    fn test_gen_sql() {
        let query =
            gen_query_verfy_all_datasource_id_exist(0, [1, 2, 3, 4, 56, 7]);
        let sql_str = query.to_string(MySqlQueryBuilder::default());

        println!("{sql_str}")
    }
}
