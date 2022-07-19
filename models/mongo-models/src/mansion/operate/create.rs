use mongo_connection::{get_mongo_database, MongoDbError};
use mongodb::bson::doc;

use super::MansionDataMongoOperate;
use crate::mansion::preludes::{MansionId, ModelMansion};

impl MansionDataMongoOperate {
    pub async fn create_mansion_data(
        mansion: ModelMansion,
    ) -> Result<(), MongoDbError> {
        let db = get_mongo_database();
        let MansionId { main_id, minor_id } = mansion.id;
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
            == 0;

        if check {
            db.doing::<_, ModelMansion, _, _>(|collection| {
                async move {
                    collection.insert_one(ModelMansion::from(mansion), None).await?;
                    Ok(())
                }
            }).await?;
        }
        return Ok(());
    }
}
