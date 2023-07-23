use std::ops::Deref;

use abstract_database::fetcher::FetcherDatabaseOperate;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm,
};
use status_err::StatusErr;
use thiserror::Error;

pub mod create;
pub mod delete;
pub mod retrieve;
pub mod verify;

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

pub struct ConfigOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for ConfigOperate<'db, Conn> {
    type Parent = FetcherDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for ConfigOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

pub trait ToConfig<C> {
    fn config(&self) -> ConfigOperate<'_, C>;
}

impl<C> ToConfig<C> for FetcherDatabaseOperate<'_, C> {
    fn config(&self) -> ConfigOperate<'_, C> { self.child() }
}
