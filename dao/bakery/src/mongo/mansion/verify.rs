use mongo_connection::{
    database_traits::get_connect::GetDatabaseCollection, CollectionGuard,
};
use mongodb::bson::Document;

use super::{MansionOperate, OperateResult};
use crate::bakery::mansion::preludes::ModelMansion;

impl<'db, Db> MansionOperate<'db, Db>
where
    Db: GetDatabaseCollection<ModelMansion>,
{
    /// 根据条件判断大厦是否存在
    /// params：mansion_id 大厦id
    pub async fn is_exist_mansion_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<ModelMansion>,
    ) -> OperateResult<bool> {
        Ok(collection
            .doing(|collection| collection.count_documents(filter, None))
            .await?
            > 0)
    }
}
