use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;

use super::{OperateResult, UserSqlOperate};
use crate::admin_user::models::user;

impl UserSqlOperate {
    // 删除一个用户
    pub async fn delete_one_user<'db, D>(
        db: &'db D, uid: i32,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;

        user::Entity::delete_by_id(uid).exec(db).await?;

        Ok(())
    }
}
