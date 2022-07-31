use super::{
    get_mansion_collection, MansionDataMongoOperate, OperateError,
    OperateResult,
};
use crate::bakery::mansion::{
    checked::Mansion,
    preludes::{MansionId, ModelMansion},
};

impl MansionDataMongoOperate {
    /// 更新大厦
    /// mid: 原先大厦id
    /// mansion: 大厦信息
    pub async fn update_mansion(
        mid: MansionId, mansion: Mansion,
    ) -> OperateResult<()> {
        let collection = get_mansion_collection()?;
        // 获取原先数据新增时间和修改时间
        let old_mansion_time =
            Self::get_mansion_time_by_id(&mid, &collection.with_mapping())
                .await?;

        // 检查id，确保id不重复
        if Self::is_exist_mansion_by_filter(mid.into_id_filter(), &collection)
            .await?
        {
            collection
                .doing(|collection| {
                    collection.find_one_and_replace(
                        mid.into_id_filter(),
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
            Err(OperateError::MansionIdExist(mansion.id.to_string()))
        }
    }
}
