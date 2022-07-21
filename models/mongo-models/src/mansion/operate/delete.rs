use mongo_connection::get_mongo_database;
use mongodb::bson::doc;

use super::MansionDataMongoOperate;
use crate::mansion::{
    preludes::{MansionId, ModelMansion},
    MansionDataError,
};

impl MansionDataMongoOperate {
    /// 删除大厦
    /// params：mid 大厦id
    pub async fn delete_mansion(
        mid: MansionId,
    ) -> Result<(), MansionDataError> {
        let db = get_mongo_database();
        let MansionId { main_id, minor_id } = mid;
        let filter = doc! {
            "id" : {
                "main_id":main_id
                ,
                "minor_id":minor_id as i32
            }
        };
        db.doing::<_, ModelMansion, _, _>(|collect| {
            async move { collect.delete_one(filter, None).await }
        })
        .await?;

        Ok(())
    }
}
