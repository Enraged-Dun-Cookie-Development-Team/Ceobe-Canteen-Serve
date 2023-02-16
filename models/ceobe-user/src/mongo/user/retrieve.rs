
use db_ops_prelude::{mongo_connection::{MongoDbCollectionTrait, CollectionGuard}, mongo_models::ceobe::user::models::{UserModel, UserDatasource, UserMobId}, mongodb::{bson::{Document, doc, Uuid}, options::FindOneOptions}, tap::Tap};
use tracing::info;

use super::{UserOperate, OperateResult};

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{
    /// 根据条件查询数据源配置
    pub(super) async fn find_datasource_list_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<UserDatasource>,
    ) -> OperateResult<Vec<Uuid>> {
        Ok(collection
            .doing(|collection| {
                async move {
                    let datasource_uuids = collection
                        .find_one(
                            filter,
                            FindOneOptions::builder()
                                .projection(doc! {"datasource_push":1i32})
                                .build(),
                        )
                        .await?;
                    let mut res = Vec::new();
                    if let Some(info) = datasource_uuids {
                        res = info.datasource_push
                    }
                    Ok(res)
                }
            })
            .await?
            .tap(|list| info!(mansionList.ids = ?list)))
    }

    /// 根据用户mob查询数据源配置
    pub async fn find_datasource_list_by_mob(
        &'db self, mob_id: UserMobId,
    ) -> OperateResult<Vec<Uuid>> {
        info!(user.mob_id = %mob_id.mob_id);
        let collection = self.get_collection()?;
        Self::find_datasource_list_by_filter(
            mob_id.into_id_filter(),
            &collection.with_mapping(),
        )
        .await
    }
}
