use tracing::{info, instrument};

use db_ops_prelude::{database_operates::NoConnect, sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter}, sql_models::fetcher::datasource_combination::models::model_datasource_combination::{Column, Entity}};

use super::{DatasourceCombinationOperate, OperateResult};

impl DatasourceCombinationOperate<'_, NoConnect> {
    /// 根据组合id删除数据
    #[instrument(ret, skip(db))]
    pub async fn delete_by_datasource(
        db: &impl ConnectionTrait, comb_ids: Vec<String>,
    ) -> OperateResult<()> {
        info! {
            datasourceCombDelete.comb_ids = ?comb_ids
        };
        Entity::delete_many()
            .filter(Column::CombinationId.is_in(comb_ids))
            .exec(db)
            .await?;

        Ok(())
    }
}
