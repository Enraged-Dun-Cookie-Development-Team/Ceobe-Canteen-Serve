use std::ops::Deref;

use abstract_database::fetcher::FetcherDatabaseOperate;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm,
};
use status_err::StatusErr;
use thiserror::Error;

pub mod retrieve;
pub mod update;

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}

type OperateResult<T> = Result<T, OperateError>;

pub struct GlobalOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for GlobalOperate<'db, Conn> {
    type Parent = FetcherDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for GlobalOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

pub trait ToGlobal<C> {
    fn global(&self) -> GlobalOperate<'_, C>;
}

impl<C> ToGlobal<C> for FetcherDatabaseOperate<'_, C> {
    fn global(&self) -> GlobalOperate<'_, C> { self.child() }
}
