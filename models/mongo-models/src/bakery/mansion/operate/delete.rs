use mongo_connection::MongoDbCollectionTrait;

use super::{MansionDataMongoOperate, OperateResult};
use crate::bakery::mansion::preludes::{MansionId, ModelMansion};

impl MansionDataMongoOperate {
    /// 删除大厦
    /// params：mid 大厦id
    pub async fn delete_mansion<'db>(
        db: &'db impl MongoDbCollectionTrait<'db, ModelMansion>,
        mid: &MansionId,
    ) -> OperateResult<()> {
        let collect = db.get_collection()?;

        collect
            .doing(|collect| collect.delete_one(mid.into_id_filter(), None))
            .await?;

        Ok(())
    }
}
