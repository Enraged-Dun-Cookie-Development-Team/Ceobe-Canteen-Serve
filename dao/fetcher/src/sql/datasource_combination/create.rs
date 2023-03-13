use db_ops_prelude::{get_connect::{GetDatabaseConnect}, sea_orm::{ConnectionTrait, ActiveModelTrait}, sql_models::fetcher::datasource_combination::models::model_datasource_combination::{ActiveModel}};
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
        &self, comb_id: String, bitmaps: [u64; 4]
    ) -> OperateResult<()> {
        info!(
            datasourceCombCreate.comb_id = comb_id,
            datasourceCombCreate.bitmap1 = bitmaps[0],
            datasourceCombCreate.bitmap2 = bitmaps[1],
            datasourceCombCreate.bitmap3 = bitmaps[2],
            datasourceCombCreate.bitmap4 = bitmaps[3],
        );

        let db = self.get_connect();

        ActiveModel::new(comb_id, bitmaps).save(db)
        .await?;
            
        Ok(())
    }
}
