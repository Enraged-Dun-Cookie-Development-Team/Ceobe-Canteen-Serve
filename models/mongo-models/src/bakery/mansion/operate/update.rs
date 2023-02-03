use mongo_connection::MongoDbCollectionTrait;
use tracing::{info, instrument, warn};

use super::{MansionOperate, OperateError, OperateResult};
use crate::bakery::mansion::{
    checked::Mansion,
    preludes::{MansionId, ModelMansion},
};

impl<'db, Db> MansionOperate<'db, Db>
where
    Db: MongoDbCollectionTrait<'db, ModelMansion>,
{
    /// 更新大厦
    /// mid: 原先大厦id
    /// mansion: 大厦信息
    #[instrument(skip(self), ret)]
    pub async fn update(
        &'db self, mid: MansionId, mansion: Mansion,
    ) -> OperateResult<()> {
        info!(
            mansionUpdate.id = %mid,
            mansionUpdate.description = mansion.description
        );

        let collection = self.get_collection()?;
        // 获取原先数据新增时间和修改时间
        let old_mansion_time =
            Self::get_mansion_time_by_id(&mid, &collection.with_mapping())
                .await?;

        // 检查id，确保id存在
        let true = Self::is_exist_mansion_by_filter(mid.into_id_filter(), &collection)
        .await? else {
            warn!(mansion.id = %mid,mansion.id.exist = false);
            return  Err(OperateError::MansionIdExist(mansion.id.to_string()));
        };

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
}
