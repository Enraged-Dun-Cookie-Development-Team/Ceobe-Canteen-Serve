use mongo_connection::MongoDbCollectionTrait;
use tracing::{info, instrument};

use super::{MansionOperate, OperateResult};
use crate::bakery::mansion::preludes::{MansionId, ModelMansion};

impl<'db, Db> MansionOperate<'db, Db>
where
    Db: MongoDbCollectionTrait<'db, ModelMansion>, {
    /// 删除大厦
    /// params：mid 大厦id
    #[instrument(skip(self))]
    pub async fn delete(
        &'db self,
        mid: &MansionId,
    ) -> OperateResult<()> {
        info!(deleteMansion.id = %mid);
        let collect = self.get_collection()?;

        let resp = collect
            .doing(|collect| collect.delete_one(mid.into_id_filter(), None))
            .await?;
        info!(deleteMansion.count = resp.deleted_count);
        Ok(())
    }
}
