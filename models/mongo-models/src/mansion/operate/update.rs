use mongodb::bson::doc;

use super::{get_mansion_collection, MansionDataMongoOperate};
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
        let collection = get_mansion_collection()?;
        // 获取原先数据新增时间和修改时间
        let old_mansion_time =
            Self::get_mansion_time_by_id(mid, &collection.with_mapping())
                .await?;
        let MansionId { main_id, minor_id } = mansion.id;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        // 检查id，确保id不重复
        if Self::is_exist_mansion_by_filter(filter.clone(), &collection)
            .await?
        {
            collection
                .doing(|collection| {
                    collection.find_one_and_replace(
                        filter,
                        ModelMansion::with_modify_time(
                            mansion,
                            old_mansion_time.now_modify(),
                        ),
                        None,
                    )
                })
                .await?;
            Ok(())
        }
        else {
            Err(MansionDataError::MansionIdExist(mansion.id.to_string()))
        }
    }
}
