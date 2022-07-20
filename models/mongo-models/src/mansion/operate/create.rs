use mongo_connection::get_mongo_database;

use super::MansionDataMongoOperate;
use crate::mansion::{preludes::ModelMansion, MansionDataError, check::Mansion};

impl MansionDataMongoOperate {
    /// 新建饼学大厦
    /// params：mansion 大厦信息
    pub async fn create_mansion_data(
        mansion: Mansion,
    ) -> Result<(), MansionDataError> {
        let db = get_mongo_database();

        // 判断mansion id是否已经存在
        let check = Self::is_exist_mansion_id(mansion.id.clone()).await?;
        if !check {
            db.doing::<_, ModelMansion, _, _>(|collection| {
                async move {
                    collection
                        .insert_one(ModelMansion::from(mansion), None)
                        .await?;
                    Ok(())
                }
            })
            .await?;
        }
        else {
            return Err(MansionDataError::MansionIdExist {
                mansion_id: mansion.id.to_string(),
            });
        }
        Ok(())
    }
}
