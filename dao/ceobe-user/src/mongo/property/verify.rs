use db_ops_prelude::{
    mongo_connection::{CollectionGuard, MongoDbCollectionTrait},
    mongo_models::ceobe::user_property::models::UserPropertyModel,
    mongodb::bson::doc,
};

use super::{OperateResult, UserPropertyOperate};

impl<'db, Conn> UserPropertyOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserPropertyModel>,
{
    /// 判断用户是否存在
    /// params：mob_id 用户mob_id
    pub async fn is_exist_user_by_db(
        mob_id: &str, collection: &CollectionGuard<UserPropertyModel>,
    ) -> OperateResult<bool> {
        Ok(collection
            .doing(|collection| {
                collection.count_documents(
                    doc! {
                        "mob_id": mob_id
                    },
                    None,
                )
            })
            .await?
            > 0)
    }

    /// 判断用户是否存在
    /// params：mob_id 用户mob_id
    pub async fn is_exist_user(
        &'db self, mob_id: &str,
    ) -> OperateResult<bool> {
        let collection = self.get_collection()?;
        Ok(collection
            .doing(|collection| {
                collection.count_documents(
                    doc! {
                        "mob_id": mob_id
                    },
                    None,
                )
            })
            .await?
            > 0)
    }
}
