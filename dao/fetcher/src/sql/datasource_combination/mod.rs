pub mod update;
pub mod create;
pub mod delete;
pub mod retrieve;
pub mod verify;
use std::ops::Deref;

use abstract_database::fetcher::FetcherDatabaseOperate;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm,
};
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("数据源组合：{0} 已经存在")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0019,
        http_code = "HttpCode::CONFLICT"
    ))]
    DatasourceCombExist(String),
    #[error("数据源超出256个")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x001A,
        resp_msg = "数据源超出上限，请联系管理员"
    ))]
    LargeThen256,
}

type OperateResult<T> = Result<T, OperateError>;

pub struct DatasourceCombinationOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for DatasourceCombinationOperate<'db, Conn> {
    type Parent = FetcherDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for DatasourceCombinationOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

pub trait ToDatasourceCombination<C> {
    fn datasource_combination(&self) -> DatasourceCombinationOperate<'_, C>;
}

impl<C> ToDatasourceCombination<C> for FetcherDatabaseOperate<'_, C> {
    fn datasource_combination(&self) -> DatasourceCombinationOperate<'_, C> {
        self.child()
    }
}
