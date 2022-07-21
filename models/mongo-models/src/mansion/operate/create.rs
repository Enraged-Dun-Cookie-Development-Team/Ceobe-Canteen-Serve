use mongo_connection::get_mongo_database;
use mongodb::bson::doc;

use super::MansionDataMongoOperate;
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
        let db = get_mongo_database();
        let MansionId { main_id, minor_id } = mansion.id;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };

        // 判断mansion id是否已经存在
        if !Self::is_exist_mansion_by_filter(filter.clone()).await? {
            db.doing::<_, ModelMansion, _, _>(|collection| {
                async move {
                    collection
                        .insert_one(ModelMansion::from(mansion), None)
                        .await
                        .map(|_| ())
                }
            })
            .await
            .map_err(Into::into)
        }
        else {
            Err(MansionDataError::MansionIdExist {
                mansion_id: mansion.id.to_string(),
            })
        }
    }
}
