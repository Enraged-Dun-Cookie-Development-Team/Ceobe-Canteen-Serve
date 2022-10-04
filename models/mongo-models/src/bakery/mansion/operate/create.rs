use mongo_connection::MongoDbCollectionTrait;

use super::{MansionDataMongoOperate, OperateError, OperateResult};
use crate::bakery::mansion::{checked::Mansion, preludes::ModelMansion};

impl MansionDataMongoOperate {
    /// 新建饼学大厦
    /// params：mansion 大厦信息
    pub async fn create_mansion_data<'db>(
        db: &'db impl MongoDbCollectionTrait<'db, ModelMansion>,
        mansion: Mansion,
    ) -> OperateResult<()> {
        let collection = db.get_collection()?;

        // 判断mansion id是否已经存在
        if !Self::is_exist_mansion_by_filter(
            mansion.id.into_id_filter(),
            &collection,
        )
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
            Err(OperateError::MansionIdExist(mansion.id.to_string()))
        }
    }
}
