use mongo_connection::get_mongo_database;
use mongodb::bson::doc;

use super::MansionDataMongoOperate;
use crate::mansion::{
    checked::Mansion,
    preludes::{MansionId, ModelMansion},
    MansionDataError,
};

impl MansionDataMongoOperate {
    /// 更新大厦
    /// mid: 原先大厦id
    /// mansion: 大厦信息
    pub async fn update_mansion(
        mid: MansionId, mansion: Mansion,
    ) -> Result<(), MansionDataError> {
        let db = get_mongo_database();
        // 获取原先数据新增时间和修改时间
        let old_mansion_time = Self::get_mansion_time_by_id(mid).await?;
        let MansionId { main_id, minor_id } = mansion.id;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        // 检查id，确保id不重复
        if Self::is_exist_mansion_by_filter(filter.clone()).await? {
            db.doing::<_, ModelMansion, _, _>(|collection| {
                async move {
                    collection
                        .find_one_and_replace(
                            filter,
                            ModelMansion::with_modify_time(
                                mansion,
                                old_mansion_time.now_modify(),
                            ),
                            None,
                        )
                        .await
                        .map(|_| ())
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
