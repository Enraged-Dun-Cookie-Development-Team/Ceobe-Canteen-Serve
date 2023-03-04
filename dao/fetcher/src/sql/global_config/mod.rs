pub mod retrieve;
pub mod update;

use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::GetDatabaseConnect,
};
use status_err::StatusErr;
use thiserror::Error;

use crate::fetcher::FetcherOperate;

pub struct GlobalOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for GlobalOperate<'db, Conn> {
    type Parent = FetcherDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for GlobalOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> FetcherDatabaseOperate<'db, Conn> {
    pub fn global(&self) -> GlobalOperate<'_, Conn> { self.child() }
}