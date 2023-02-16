
use db_ops_prelude::{mongo_connection::MongoDbCollectionTrait, mongo_models::ceobe::user::{models::{UserChecked, UserModel}}};
use tracing::{info, instrument, warn};
use crate::user::{OperateResult, OperateError};

use super::UserOperate;

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{
    /// 新建蹲饼用户
    /// params 用户初始信息
    #[instrument(skip(self), ret)]
    pub async fn create(&'db self, user: UserChecked) -> OperateResult<()> {
        info!(
            newUser.mob_id = %user.mob_id,
            newUser.datasource_push = ?user.datasource_push
        );

        let collection = self.get_collection()?;

        // 判断用户 id是否已经存在
        let false = Self::is_exist_user_by_db(
            &user.mob_id,
            &collection,
        )
        .await? else {
            warn!(newUser.mob_id = %user.mob_id, newUser.mob_id.exist = true);
            return Err(OperateError::UserMobIdExist(user.mob_id))
        };

        // 将用户初始化信息存入数据库
        collection
            .doing(|collection| {
                collection.insert_one(UserModel::from(user), None)
            })
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}
