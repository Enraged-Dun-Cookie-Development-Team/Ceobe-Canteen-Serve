use db_ops_prelude::{
    bool_or::FalseOrError,
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::user_property::models::{
        UserPropertyChecked, UserPropertyModel,
    },
};
use tracing::{info, instrument, warn};

use super::UserPropertyOperate;
use crate::property::{OperateError, OperateResult};

impl<'db, Conn> UserPropertyOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserPropertyModel>,
{
    /// 新建蹲饼用户
    /// params 用户初始信息
    #[instrument(skip(self), ret)]
    pub async fn create(
        &'db self, user: UserPropertyChecked,
    ) -> OperateResult<()> {
        info!(
            newUser.mob_id = %user.mob_id,
            newUser.datasource_push = ?user.datasource_push
        );

        let collection = self.get_collection()?;

        // 判断用户 id是否已经存在
        Self::is_exist_user_by_db(
            &user.mob_id,
            &collection,
        )
        .await?.false_or_with(|| {
            warn!(newUser.mob_id = %user.mob_id, newUser.mob_id.exist = true);
            OperateError::UserMobIdExist(user.mob_id.clone())
        })?;

        // 将用户初始化信息存入数据库
        collection
            .doing(|collection| {
                collection.insert_one(UserPropertyModel::from(user), None)
            })
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}
