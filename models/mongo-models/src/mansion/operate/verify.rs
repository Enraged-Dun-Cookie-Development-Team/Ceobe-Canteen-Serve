use mongo_connection::CollectionGuard;
use mongodb::bson::Document;

use super::MansionDataMongoOperate;
use crate::mansion::{preludes::ModelMansion, MansionDataError};

impl MansionDataMongoOperate {
    /// 根据条件判断大厦是否存在
    /// params：mansion_id 大厦id
    pub async fn is_exist_mansion_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<ModelMansion>,
    ) -> Result<bool, MansionDataError> {
        Ok(collection
            .doing(|collection| collection.count_documents(filter, None))
            .await?
            > 0)
    }
}
