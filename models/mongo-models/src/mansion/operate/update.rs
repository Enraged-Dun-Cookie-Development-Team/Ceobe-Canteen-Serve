use mongo_connection::get_mongo_database;
use mongodb::bson::doc;

use crate::mansion::{preludes::{ModelMansion, MansionId}, MansionDataError, check::Mansion};

use super::MansionDataMongoOperate;



impl MansionDataMongoOperate {
    /// 更新大厦 
    /// mid: 原先大厦id
    /// mansion: 大厦信息
    pub async fn update_mansion(
        mid: MansionId, mansion:Mansion
    ) -> Result<(), MansionDataError> {
        let db = get_mongo_database();
        // 获取原先数据新增时间和修改时间
        let old_mansion_time = Self::get_mansion_time_by_id(mid).await?;
        let MansionId { main_id, minor_id} = mansion.id;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        // 检查id，确保id不重复
        let check = Self::is_exist_mansion_id(mansion.id.clone()).await?;
        if check {
            db.doing::<_, ModelMansion, _, _>(|collection| {
                async move {
                    let task = collection.find_one_and_replace(
                        filter,
                        ModelMansion::with_modify_time(
                            mansion.into(),
                            old_mansion_time.now_modify(),
                        ),
                        None,
                    );
                    task.await?;
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