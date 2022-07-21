use mongodb::bson::doc;

use super::{get_mansion_collection, MansionDataMongoOperate};
use crate::mansion::{
    checked::Mansion,
    preludes::{MansionId, ModelMansion},
    MansionDataError,
};

impl MansionDataMongoOperate {
    /// 新建饼学大厦
    /// params：mansion 大厦信息
    pub async fn create_mansion_data(
        mansion: Mansion,
    ) -> Result<(), MansionDataError> {
        let collection = get_mansion_collection()?;
        let MansionId { main_id, minor_id } = mansion.id;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };

        // 判断mansion id是否已经存在
        if !Self::is_exist_mansion_by_filter(filter.clone(), &collection)
            .await?
        {
            collection
                .doing(|collection| {
                    collection.insert_one(ModelMansion::from(mansion), None)
                })
                .await
                .map(|_| ())
                .map_err(Into::into)
        }
        else {
            Err(MansionDataError::MansionIdExist(mansion.id.to_string()))
        }
    }
}
