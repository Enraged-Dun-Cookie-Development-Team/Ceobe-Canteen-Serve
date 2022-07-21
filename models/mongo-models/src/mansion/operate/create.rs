use super::{get_mansion_collection, MansionDataMongoOperate};
use crate::mansion::{
    checked::Mansion, preludes::ModelMansion, MansionDataError,
};

impl MansionDataMongoOperate {
    /// 新建饼学大厦
    /// params：mansion 大厦信息
    pub async fn create_mansion_data(
        mansion: Mansion,
    ) -> Result<(), MansionDataError> {
        let collection = get_mansion_collection()?;

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
            Err(MansionDataError::MansionIdExist(mansion.id.to_string()))
        }
    }
}
