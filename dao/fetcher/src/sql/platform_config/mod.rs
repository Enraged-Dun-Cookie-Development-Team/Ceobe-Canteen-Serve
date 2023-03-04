pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod verify;

use db_ops_prelude::{sea_orm::FromQueryResult, database_operates::sub_operate::{SubOperate, SuperOperate}};
use std::ops::Deref;
use abstract_database::fetcher::FetcherDatabaseOperate;
use db_ops_prelude::sea_orm;
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;


#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("该平台下存在数据源，无法删除平台")]
    #[status_err(err(
        err_code = 0x00_12,
        prefix = "ErrPrefix::CHECKER",
        http_code = "HttpCode::CONFLICT"
    ))]
    NoDeletePlatformHasDatasource,
}

type OperateResult<T> = Result<T, OperateError>;

pub struct PlatformOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for PlatformOperate<'db, Conn> {
    type Parent = FetcherDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for PlatformOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

pub trait ToPlatform<C> {
    fn platform(&self) -> PlatformOperate<'_, C>;
}

impl<C> ToPlatform<C> for FetcherDatabaseOperate<'_, C> {
    fn platform(&self) -> PlatformOperate<'_, C> { self.child() }
}