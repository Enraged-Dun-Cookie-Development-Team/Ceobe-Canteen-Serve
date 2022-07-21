use mongodb::bson::doc;

use super::{get_mansion_collection, MansionDataMongoOperate};
use crate::mansion::{preludes::MansionId, MansionDataError};

impl MansionDataMongoOperate {
    /// 删除大厦
    /// params：mid 大厦id
    pub async fn delete_mansion(
        mid: MansionId,
    ) -> Result<(), MansionDataError> {
        let collect = get_mansion_collection()?;
        let MansionId { main_id, minor_id } = mid;
        let filter = doc! {
            "id" : {
                "main_id":main_id
                ,
                "minor_id":minor_id as i32
            }
        };

        collect.doing(|collect| collect.delete_one(filter, None)).await?;

        Ok(())
    }
}
