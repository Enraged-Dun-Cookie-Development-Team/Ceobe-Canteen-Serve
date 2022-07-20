use mongo_connection::get_mongo_database;
use mongodb::bson::doc;

use super::MansionDataMongoOperate;
use crate::mansion::{
    preludes::{MansionId, ModelMansion},
    MansionDataError,
};

impl MansionDataMongoOperate {
    /// 查询id是否存在
    /// params：mansion_id 大厦id
    pub async fn is_exist_mansion_id(
        mid: MansionId,
    ) -> Result<bool, MansionDataError> {
        let db = get_mongo_database();
        let MansionId { main_id, minor_id } = mid;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        let check = db
            .doing::<_, ModelMansion, _, _>(|collection| {
                async move { collection.count_documents(filter, None).await }
            })
            .await?
            > 0;
        Ok(check)
    }
}
