use bool_or::FalseOrError;
use mongo_connection::MongoDbCollectionTrait;
use tracing::{info, instrument, warn};

use super::{MansionOperate, OperateError, OperateResult};
use crate::bakery::mansion::{checked::Mansion, preludes::ModelMansion};

impl<'db, Db> MansionOperate<'db, Db>
where
    Db: MongoDbCollectionTrait<'db, ModelMansion>,
{
    /// 新建饼学大厦
    /// params：mansion 大厦信息
    #[instrument(skip(self), ret)]
    pub async fn create(&'db self, mansion: Mansion) -> OperateResult<()> {
        info!(
            newMansion.id = %mansion.id,
            newMansion.description = mansion.description
        );

        let collection = self.get_collection()?;

        // 判断mansion id是否已经存在
        Self::is_exist_mansion_by_filter(
            mansion.id.into_id_filter(),
            &collection,
        )
        .await?
        .false_or_with(|| {
            warn!(newMansion.id = %mansion.id, newMansion.id.exist = true);
            OperateError::MansionIdExist(mansion.id.to_string())
        })?;

        collection
            .doing(|collection| {
                collection.insert_one(ModelMansion::from(mansion), None)
            })
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}
