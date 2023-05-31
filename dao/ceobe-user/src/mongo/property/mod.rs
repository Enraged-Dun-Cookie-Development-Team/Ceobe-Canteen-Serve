pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod verify;

use std::ops::Deref;

use db_ops_prelude::mongo_connection;
use mongo_connection::{
    database_traits::database_operates::sub_operate::{
        SubOperate, SuperOperate,
    },
    MongoDbError,
};
use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

use crate::UserDatabaseOperate;

pub struct UserPropertyOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for UserPropertyOperate<'db, Conn> {
    type Parent = UserDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for UserPropertyOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

type OperateResult<T> = Result<T, OperateError>;

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("Mongo数据库异常: {0}")]
    Db(#[from] MongoDbError),

    #[error("用户Mob ID:[{0:?}] 已经存在")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0018,
        http_code = "HttpCode::CONFLICT"
    ))]
    UserMobIdExist(String),

    #[error("用户Mob ID:{0:?} 不存在")]
    #[status_err(err(
        prefix = "ErrPrefix::NOT_FOUND",
        err_code = 0x0009,
        resp_msg = "Mob Id不存在，请加群联系管理"
    ))]
    UserMobIdNotExist(String),
}

impl<'db, Conn> UserDatabaseOperate<'db, Conn> {
    pub fn property(&self) -> UserPropertyOperate<'_, Conn> { self.child() }
}
