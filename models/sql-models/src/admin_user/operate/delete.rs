use sea_orm::{ConnectionTrait, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};

use super::{OperateResult, UserOperate};
use crate::admin_user::models::user;

impl<'c, C> UserOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self), ret)]
    // 删除一个用户
    pub async fn delete_one(&self, uid: i32) -> OperateResult<()> {
        info!(user.id = uid);
        let db = self.get_connect();

        user::Entity::delete_by_id(uid).exec(db).await?;

        Ok(())
    }
}
