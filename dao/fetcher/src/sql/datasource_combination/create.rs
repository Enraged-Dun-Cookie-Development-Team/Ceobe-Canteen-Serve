use db_ops_prelude::{get_connect::{GetDatabaseConnect}, sea_orm::{ConnectionTrait, ActiveModelTrait}, sql_models::fetcher::datasource_combination::models::model_datasource_combination::{CombinationInfo}};
use tracing::{info, instrument};

use super::{DatasourceCombinationOperate, OperateResult};

impl<'c, C> DatasourceCombinationOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(ret, skip(self))]
    /// 创建数据源组合数据
    pub async fn create(
        &self, comb_id_info: CombinationInfo,
    ) -> OperateResult<()> {
        info!(
            datasourceCombCreate.comb_id = comb_id_info.combination_id,
            datasourceCombCreate.bitmap1 = comb_id_info.bitmap1,
            datasourceCombCreate.bitmap2 = comb_id_info.bitmap1,
            datasourceCombCreate.bitmap3 = comb_id_info.bitmap1,
            datasourceCombCreate.bitmap4 = comb_id_info.bitmap1,
        );

        let db = self.get_connect();

        comb_id_info
            .into_active_model_with_access_time()
            .save(db)
            .await?;
        Ok(())
    }
}
