use mongo_connection::get_mongo_database;
use mongodb::bson::{Document};

use super::MansionDataMongoOperate;
use crate::mansion::{preludes::ModelMansion, MansionDataError};

impl MansionDataMongoOperate {
    /// 根据条件判断大厦是否存在
    /// params：mansion_id 大厦id
    pub async fn is_exist_mansion_by_filter(
        filter: impl Into<Option<Document>>,
    ) -> Result<bool, MansionDataError> {
        let db = get_mongo_database();
        let check = db
            .doing::<_, ModelMansion, _, _>(|collection| {
                async move { collection.count_documents(filter, None).await }
            })
            .await?
            > 0;
        Ok(check)
    }
}
