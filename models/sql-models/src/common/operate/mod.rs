mod modify_user;
mod query_user;
mod user_count;
use sea_orm::FromQueryResult;

mod add_user;

pub struct CommonSqlOperate;

#[derive(FromQueryResult)]
struct UserCounts {
    pub(crate) count: i64,
}
