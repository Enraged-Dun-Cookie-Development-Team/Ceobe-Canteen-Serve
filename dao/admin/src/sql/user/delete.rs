use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{ConnectionTrait, EntityTrait},
    sql_models::admin_user,
};
use tracing::{info, instrument};

use super::{OperateResult, UserOperate};

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

        admin_user::Entity::delete_by_id(uid).exec(db).await?;

        Ok(())
    }
}
