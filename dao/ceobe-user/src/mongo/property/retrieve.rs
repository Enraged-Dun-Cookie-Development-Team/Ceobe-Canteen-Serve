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

    /// 根据数据源查询用户列表
    pub async fn find_user_list_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<UserMobId>,
    ) -> OperateResult<Vec<String>> {
        let res = collection
            .doing(|collection| {
                collection.find(
                    filter,
                    FindOptions::builder()
                        .projection(doc! {"mob_id":1i32})
                        .build(),
                )
            })
            .await?
            .map_ok(|id| id.mob_id)
            .try_collect::<Vec<String>>()
            .await
            .map_err(MongoDbError::from)?;
        Ok(res)
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

    /// 根据数据源查询用户列表
    pub async fn find_user_list_by_datasource(
        &'db self, datasource_uuid: Uuid,
    ) -> OperateResult<Vec<String>> {
        info!(user.mob_id = %datasource_uuid);
        let collection = self.get_collection()?;
        Self::find_user_list_by_filter(
            doc! {"datasource_push": datasource_uuid},
            &collection.with_mapping(),
        )
        .await
    }
}
