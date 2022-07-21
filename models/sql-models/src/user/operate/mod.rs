mod update;
mod retrieve;
mod verify;
use sea_orm::FromQueryResult;

mod create;

pub struct CommonSqlOperate;

#[derive(FromQueryResult)]
struct UserCounts {
    pub(crate) count: i64,
}
