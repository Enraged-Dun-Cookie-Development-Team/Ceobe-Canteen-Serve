use mongo_connection::MongoDbCollectionTrait;
use tracing::{info, instrument};

use super::{MansionDataMongoOperate, OperateResult};
use crate::bakery::mansion::preludes::{MansionId, ModelMansion};

impl MansionDataMongoOperate {
    /// 删除大厦
    /// params：mid 大厦id
    #[instrument(skip(db))]
    pub async fn delete_mansion<'db>(
        db: &'db impl MongoDbCollectionTrait<'db, ModelMansion>,
        mid: &MansionId,
    ) -> OperateResult<()> {
        info!(deleteMansion.id = %mid);
        let collect = db.get_collection()?;

        let resp = collect
            .doing(|collect| collect.delete_one(mid.into_id_filter(), None))
            .await?;
        info!(deleteMansion.count = resp.deleted_count);
        Ok(())
    }
}
