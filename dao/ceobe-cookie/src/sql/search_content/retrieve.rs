use db_ops_prelude::{get_connect::GetDatabaseConnect, sea_orm::{ConnectionTrait, StreamTrait, EntityTrait, sea_query::{Query, Expr, Func, SelectStatement}, ColumnTrait, Order, DatabaseBackend, StatementBuilder}, sql_models::ceobe_cookie::search_content::models::model_search_content::{Entity, SearchOid, Column}, mysql_func::AGAINST};
use tracing::instrument;

use super::{SearchContentOperate, OperateResult};

impl<'c, C> SearchContentOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    pub async fn find_page_object_ids(&self, object_id: Option<String>, search_word: &str, datasources: Vec<i32>, page_size: u64) -> OperateResult<Vec<String>> {
        let db: &<C as GetDatabaseConnect>::Connect = self.get_connect();
        let mut query = gen_search_content_condition_sql(object_id,search_word, datasources);
        query.limit(page_size);
        let query = StatementBuilder::build(&query, &DatabaseBackend::MySql);
        Ok(Entity::find()
            .from_raw_sql(query)
            .into_model::<SearchOid>()
            .all(db)
            .await?
            .into_iter()
            .map(|item| item.object_id)
            .collect())
        
    }

    #[instrument(skip(self))]
    pub async fn find_next_page_object_id(&self, object_id: Option<String>, search_word: &str, datasources: Vec<i32>, page_size: u64) -> OperateResult<Option<String>> {
        let db: &<C as GetDatabaseConnect>::Connect = self.get_connect();
        let mut query = gen_search_content_condition_sql(object_id,search_word, datasources);
        query.limit(1).offset(page_size);
        let query = StatementBuilder::build(&query, &DatabaseBackend::MySql);
        Ok(Entity::find()
                .from_raw_sql(query)
                .into_model::<SearchOid>()
                .one(db)
                .await?
                .map(|item| item.object_id))
    }
}

pub fn gen_search_content_condition_sql(object_id: Option<String>, search_word: &str, datasources: Vec<i32>) -> SelectStatement {
    let mut query = Query::select();
        // SELECT combination_id
        query.expr(Expr::col(
            Column::ObjectId,
        ));

        // FROM fetcher_datasource_combination
        query.from(Entity);

        if let Some(object_id_str) = object_id {
            query.and_where(Column::ObjectId.lte(object_id_str));
        }
        
        query.and_where(Column::SourceConfigId.is_in(datasources));

        query.and_where(Expr::cust_with_exprs("MATCH ? ?", [Expr::cust("content"), Func::cust(AGAINST).arg(search_word).into()]));

        query.order_by(Column::ObjectId, Order::Desc);
        query
}