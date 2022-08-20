use super::{get_mansion_collection, MansionDataMongoOperate, OperateResult};
use crate::bakery::mansion::preludes::MansionId;

impl MansionDataMongoOperate {
    /// 删除大厦
    /// params：mid 大厦id
    pub async fn delete_mansion(mid: &MansionId) -> OperateResult<()> {
        let collect = get_mansion_collection()?;

        collect
            .doing(|collect| collect.delete_one(mid.into_id_filter(), None))
            .await?;

        Ok(())
    }
}
