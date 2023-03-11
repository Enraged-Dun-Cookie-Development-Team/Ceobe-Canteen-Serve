use db_ops_prelude::{
    futures::TryStreamExt,
    mongo_connection::{
        CollectionGuard, MongoDbCollectionTrait, MongoDbError,
    },
    mongo_models::ceobe::user_property::models::{
        UserDatasource, UserMobId, UserPropertyModel,
    },
    mongodb::{
        bson::{doc, Document, Uuid},
        options::{FindOneOptions, FindOptions},
    },
    tap::Tap,
};
use tracing::info;

use super::{OperateResult, UserPropertyOperate};

impl<'db, Conn> UserPropertyOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserPropertyModel>,
{
    /// 根据条件查询数据源配置
    pub(super) async fn find_datasource_list_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<UserDatasource>,
    ) -> OperateResult<Vec<Uuid>> {
        Ok(collection
            .doing(|collection| {
                collection.find_one(
                    filter,
                    FindOneOptions::builder()
                        .projection(doc! {"datasource_push":1i32})
                        .build(),
                )
            })
            .await?
            .map(|info| info.datasource_push)
            .unwrap_or_default()
            .tap(|list| info!(mansionList.ids = ?list)))
    }

    /// 根据用户mob查询数据源配置
    pub async fn find_datasource_list_by_mob(
        &'db self,
        mob_id: UserMobId,
    ) -> OperateResult<Vec<Uuid>> {
        info!(user.mob_id = %mob_id.mob_id);
        let collection = self.get_collection()?;
        Self::find_datasource_list_by_filter(
            mob_id.into_id_filter(),
            &collection.with_mapping(),
        )
        .await
    }

    pub async fn find_all_subscribed_user_mob_id(
        &'db self,
        datasource_id: Uuid,
    ) -> OperateResult<Vec<UserMobId>> {
        info!(datasource.uuid = %datasource_id);

        let collection = self.get_collection()?;

        let v = collection
            .with_mapping()
            .doing(|collection| {
                collection.find(
                    doc! {
                        "datasource_push": {
                            "$in": [
                                datasource_id
                                ]
                        }
                    },
                    FindOptions::builder()
                        .projection(doc! {"mob_id": 1})
                        .build(),
                )
            })
            .await?
            .map_err(MongoDbError::from)
            .try_collect()
            .await?;
        Ok(v)
    }
}
