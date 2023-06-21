use db_ops_prelude::{get_connect::GetDatabaseConnect, sea_orm::{ConnectionTrait, StreamTrait, EntityTrait, sea_query::{Expr, Func}, ColumnTrait, Order, QueryFilter, Condition, QueryOrder, QuerySelect}, sql_models::ceobe_cookie::search_content::models::model_search_content::{Entity, SearchOid, Column}, mysql_func::{AGAINST, MATCH}};
use tracing::instrument;

use super::{OperateResult, SearchContentOperate};

impl<'c, C> SearchContentOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    pub async fn get_page_cookie_ids(
        &self, object_id: Option<String>, search_word: &str,
        datasources: &[i32], page_size: u64,
    ) -> OperateResult<Vec<String>> {
        let db: &<C as GetDatabaseConnect>::Connect = self.get_connect();
        Ok(Entity::find()
            .filter(
                Condition::all()
                    .add_option(
                        object_id
                            .map(|id| Expr::col(Column::ObjectId).lte(id)),
                    )
                    .add(
                        Column::SourceConfigId
                            .is_in(datasources.iter().copied()),
                    )
                    .add(Expr::cust_with_exprs(
                        "? ?",
                        [
                            Func::cust(MATCH)
                                .arg(Expr::col(Column::Content))
                                .into(),
                            Func::cust(AGAINST).arg(search_word).into(),
                        ],
                    )),
            )
            .order_by(Column::ObjectId, Order::Desc)
            .limit(page_size)
            .into_model::<SearchOid>()
            .all(db)
            .await?
            .into_iter()
            .map(|item| item.object_id)
            .collect())
    }

    /// 获取下一页的饼id
    #[instrument(skip(self))]
    pub async fn get_next_page_cookie_id(
        &self, object_id: Option<String>, search_word: &str,
        datasources: &[i32], page_size: u64,
    ) -> OperateResult<Option<String>> {
        let db: &<C as GetDatabaseConnect>::Connect = self.get_connect();
        Ok(Entity::find()
            .select_only()
            .column(Column::ObjectId)
            .filter(
                Condition::all()
                    .add_option(
                        object_id
                            .map(|id| Expr::col(Column::ObjectId).lte(id)),
                    )
                    .add(
                        Column::SourceConfigId
                            .is_in(datasources.iter().copied()),
                    )
                    .add(Expr::cust_with_exprs(
                        "? ?",
                        [
                            Func::cust(MATCH)
                                .arg(Expr::col(Column::Content))
                                .into(),
                            Func::cust(AGAINST).arg(search_word).into(),
                        ],
                    )),
            )
            .order_by(Column::ObjectId, Order::Desc)
            .limit(1)
            .offset(page_size)
            .into_model::<SearchOid>()
            .one(db)
            .await?
            .map(|item| item.object_id))
    }
}

#[cfg(test)]
mod test {
    use db_ops_prelude::{sea_orm::{sea_query::{Query, Expr, Func, MysqlQueryBuilder}, ColumnTrait, Order}, sql_models::ceobe_cookie::search_content::models::model_search_content::{Column, Entity}, mysql_func::{AGAINST, MATCH}};

    #[test]
    fn test_gen_sql() {
        let object_id: Option<String> = None;
        let datasources = vec![1i32, 3, 4, 6];
        let search_word: &str =
            "市组准标取';Select * from ceobe_cookie_search_content;";

        let mut query = Query::select();
        // SELECT combination_id
        query.expr(Expr::col(Column::ObjectId));

        // FROM fetcher_datasource_combination
        query.from(Entity);

        if let Some(object_id_str) = object_id {
            query.and_where(Column::ObjectId.lte(object_id_str));
        }

        query.and_where(Column::SourceConfigId.is_in(datasources));

        query.and_where(Expr::cust_with_exprs(
            "? ?",
            [
                Func::cust(MATCH).arg(Expr::col(Column::Content)).into(),
                Func::cust(AGAINST).arg(search_word).into(),
            ],
        ));

        query.order_by(Column::ObjectId, Order::Desc);

        print!("{}", query.to_string(MysqlQueryBuilder))
    }
}
