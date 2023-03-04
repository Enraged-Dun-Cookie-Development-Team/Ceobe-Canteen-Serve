pub mod create;
pub mod delete;
pub mod retrieve;
pub mod verify;

use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::GetDatabaseConnect,
};
use status_err::StatusErr;
use thiserror::Error;

pub struct ConfigOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for ConfigOperate<'db, Conn> {
    type Parent = ConfigOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for ConfigOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> FetcherDatabaseOperate<'db, Conn> {
    pub fn config(&self) -> ConfigOperate<'_, Conn> { self.child() }
}