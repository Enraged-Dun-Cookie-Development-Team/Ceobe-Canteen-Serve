
use db_ops_prelude::{mongodb::bson::doc, mongo_connection::{CollectionGuard, MongoDbCollectionTrait}, mongo_models::ceobe::user::models::UserModel};

use super::{OperateResult, UserOperate};

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{
    /// 判断用户是否存在
    /// params：mob_id 用户mob_id
    pub async fn is_exist_user_by_db(
        mob_id: &str, collection: &CollectionGuard<UserModel>,
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