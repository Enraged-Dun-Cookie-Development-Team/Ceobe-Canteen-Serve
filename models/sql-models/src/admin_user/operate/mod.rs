mod retrieve;
mod update;
mod verify;
use sea_orm::FromQueryResult;

mod create;

pub struct UserSqlOperate;

#[derive(FromQueryResult)]
struct UserCounts {
    pub(crate) count: i64,
}
