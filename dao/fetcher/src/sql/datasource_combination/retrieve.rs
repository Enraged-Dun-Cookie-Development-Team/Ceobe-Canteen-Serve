use db_ops_prelude::{get_connect::GetDatabaseConnect, sea_orm::{ConnectionTrait, ActiveModelTrait, EntityTrait, QuerySelect, Condition, DbBackend, Statement}, sql_models::fetcher::datasource_combination::models::model_datasource_combination::{self, Entity, Column, CombinationId}};
use tracing::{info, instrument};

use crate::datasource_combination::OperateError;

use super::{DatasourceCombinationOperate, OperateResult};



impl<'c, C> DatasourceCombinationOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(ret, skip(self))]
    /// 根据一个数据源查找对应的数据源组合id
    pub async fn find_comb_id_by_one_datasource(
        &self, datasource_id: i32,
    ) -> OperateResult<Vec<String>> {
        info!(
            datasourceComb.datasource_id = datasource_id,
        );
        let db = self.get_connect();
        let mut sql = String::from("SELECT combination_id FROM fetcher_datasource_combination");
        
        let index: u64 = (datasource_id % 64).try_into().unwrap();
        let datasource_base2: u64 = 1 << index;
        // 获取对应的bitmap
        match datasource_id / 64  {
            0 => {sql += &format!(r#" WHERE bitmap1 & {datasource_base2} = {datasource_base2}"#)},
            1 => {sql += &format!(r#" WHERE bitmap2 & {datasource_base2} = {datasource_base2}"#)},
            2 => {sql += &format!(r#" WHERE bitmap3 & {datasource_base2} = {datasource_base2}"#)},
            3 => {sql += &format!(r#" WHERE bitmap4 & {datasource_base2} = {datasource_base2}"#)},
            _ => return Err(OperateError::LargeThen256),
        }


        Ok(Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(DbBackend::MySql, &sql, []))
            .into_model::<CombinationId>()
            .all(db)
            .await?
            .into_iter()
            .map(|id| id.combination_id)
            .collect())
    }
}