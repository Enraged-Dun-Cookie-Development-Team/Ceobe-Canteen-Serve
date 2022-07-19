use mongodb::bson::doc;

use crate::mansion::preludes::{ModelMansion, MansionId};

use super::{MansionDataMongoOperate, MongoErr};




impl MansionDataMongoOperate {
    pub async fn create_mansion_data<T>(mansion : ModelMansion) -> Result<T, MongoErr> {
        let db = set_mongo_database();
        let MansionId{ main_id, minor_id} = mansion.id;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        let check = db.doing::<_, ModelMansion, _, _>(|collection| {
            async move {
                collection.count_documents(filter, None).await
            }
        }).
        Ok(())
    }
}