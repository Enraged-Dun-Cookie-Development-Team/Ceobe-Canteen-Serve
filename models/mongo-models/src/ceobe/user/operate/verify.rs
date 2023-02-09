use crate::ceobe::user::models::UserModel;

use super::{UserOperate, OperateResult};
use mongo_connection::{MongoDbCollectionTrait, CollectionGuard};
use mongodb::bson::doc;

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{
    /// 判断用户是否存在
    /// params：mob_id 用户mob_id
    pub async fn is_exist_user(
        mob_id: &str,
        collection: &CollectionGuard<UserModel>,
    ) -> OperateResult<bool> {
        Ok(collection
            .doing(|collection| collection.count_documents(
                doc! {
                    "mob_id": mob_id
                },
                None))
            .await?
            > 0)
    }
}