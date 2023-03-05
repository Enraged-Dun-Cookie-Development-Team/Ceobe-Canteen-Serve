use std::ops::Deref;

use abstract_database::bakery::BakeryDatabaseOperate;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    mongo_connection::MongoDbError,
};

mod create;
mod delete;
mod retrieve;
mod update;
mod verify;
use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;
pub use OperateError::*;

type OperateResult<T> = Result<T, OperateError>;

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("Mongo数据库异常")]
    Db(#[from] MongoDbError),

    #[error("指定饼学大厦ID未找到")]
    #[status_err(err(prefix = "ErrPrefix::NOT_FOUND", err_code = 0x0001,))]
    MansionNotFound,
    #[error("指定ID:[{0:?}] 的饼学大厦已经存在")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0008,
        http_code = "HttpCode::CONFLICT"
    ))]
    MansionIdExist(String),
}

pub struct MansionOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for MansionOperate<'db, Conn> {
    type Parent = BakeryDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for MansionOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

pub trait ToMansion<C> {
    fn mansion(&self) -> MansionOperate<'_, C>;
}

impl<C> ToMansion<C> for BakeryDatabaseOperate<'_, C> {
    fn mansion(&self) -> MansionOperate<'_, C> { self.child() }
}
