use sea_orm::EntityTrait;
use sql_connection::get_sql_database;

use super::{OperateResult, UserSqlOperate};
use crate::admin_user::models::user;

impl UserSqlOperate {
    // 删除一个用户
    pub async fn delete_one_user(uid: i32) -> OperateResult<()> {
        let db = get_sql_database();

        user::Entity::delete_by_id(uid).exec(db).await?;

        Ok(())
    }
}
