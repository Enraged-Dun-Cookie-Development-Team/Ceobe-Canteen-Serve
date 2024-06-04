use tracing::{info, instrument};

use db_ops_prelude::{database_operates::NoConnect, get_connect::GetDatabaseConnect, sea_orm::{ConnectionTrait, EntityTrait, sea_query::{Expr, MysqlQueryBuilder, Query}, Statement}, sql_models::fetcher::datasource_combination::models::model_datasource_combination::{Column, CombinationId, Entity}};
use db_ops_prelude::sea_orm::{DerivePartialModel, FromQueryResult, IntoSimpleExpr, QueryFilter};
use db_ops_prelude::sea_orm;
use super::{DatasourceCombinationOperate, OperateResult};

impl DatasourceCombinationOperate<'_, NoConnect> {
    #[instrument(ret, skip(db))]
    /// 根据数据源查询数据源组合
    pub async fn find_comb_id_by_one_datasource_raw(
        db: &impl ConnectionTrait, datasource_id: i32,
    ) -> OperateResult<Vec<String>> {
        info!(datasourceComb.datasource_id = datasource_id,);

        let mut query = Query::select();

        // SELECT combination_id
        query.expr(Expr::col(Column::CombinationId));

        // FROM fetcher_datasource_combination
        query.from(Entity);
        let index: u64 = (datasource_id % 64).try_into().unwrap();
        let datasource_base2: u64 = 1 << index;
        let bitmap_number = (datasource_id / 64 + 1).try_into().unwrap();
        // where
        query.and_where(Expr::cust_with_values(
            "bitmap? & ? = ?",
            [bitmap_number, datasource_base2, datasource_base2],
        ));

        Ok(Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db.get_database_backend(),
                &query.to_string(MysqlQueryBuilder).to_lowercase(),
                [],
            ))
            .into_model::<CombinationId>()
            .all(db)
            .await?
            .into_iter()
            .map(|id| id.combination_id)
            .collect())
    }
}

impl<'c, C> DatasourceCombinationOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    /// 根据一个数据源查找对应的数据源组合id
    pub async fn find_comb_id_by_one_datasource(
        &self, datasource_id: i32,
    ) -> OperateResult<Vec<String>> {
        info!(datasourceComb.datasource_id = datasource_id,);
        let db = self.get_connect();
        DatasourceCombinationOperate::find_comb_id_by_one_datasource_raw(
            db,
            datasource_id,
        )
        .await
    }

    pub async fn find_expired(
        &self, expired_days: i32,
    ) -> Result<Vec<String>, super::OperateError> {
        #[derive(Debug, FromQueryResult, DerivePartialModel)]
        #[sea_orm(entity = "Entity")]
        struct ExpiredId {
            #[sea_orm(from_col = "combination_id")]
            combine_id: String,
        }

        let result = Entity::find()
            .filter(
                Expr::expr(
                    Expr::current_date()
                        .sub(Column::LastAccessTime.into_simple_expr()),
                )
                .lte(Expr::val(expired_days)),
            )
            .into_partial_model::<ExpiredId>()
            .all(self.get_connect())
            .await?
            .into_iter()
            .map(|ExpiredId { combine_id }| combine_id)
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use db_ops_prelude::{
        sea_orm::sea_query::{
            Expr, MysqlQueryBuilder, Query, SelectStatement,
        },
        sql_models::fetcher::datasource_combination::models::model_datasource_combination,
    };

    /// 生成Statement 为 检查给定的id的平台下时候有数据源
    fn gen_query_find_comb_id_by_one_datasource(
        datasource_id: i32,
    ) -> SelectStatement {
        let mut query = Query::select();

        // SELECT combination_id
        query.expr(Expr::col(
            model_datasource_combination::Column::CombinationId,
        ));

        // FROM fetcher_datasource_combination
        query.from(model_datasource_combination::Entity);
        let index: u64 = (datasource_id % 64).try_into().unwrap();
        let datasource_base2: u64 = 1 << index;
        let bitmap_number = (datasource_id / 64 + 1).try_into().unwrap();
        // join
        query.and_where(Expr::cust_with_values(
            "bitmap? & ? = ?",
            [bitmap_number, datasource_base2, datasource_base2],
        ));

        query
    }

    #[test]
    fn test_gen_sql() {
        let query = gen_query_find_comb_id_by_one_datasource(10);
        let sql_str = query.to_string(MysqlQueryBuilder).to_lowercase();
        println!("{sql_str}");
    }
}
