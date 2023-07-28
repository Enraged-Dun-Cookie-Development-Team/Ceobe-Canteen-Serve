use std::ops::Deref;

pub use db_ops_prelude::mongo_models::ceobe::operation::plugin_version::*;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    mongo_connection::MongoDbError,
    ErrPrefix, HttpCode, StatusErr, ThisError,
};

use crate::OperationDatabaseOperate;

mod retrieve;
mod update;
mod verify;

pub struct PluginVersionOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for PluginVersionOperate<'db, Conn> {
    type Parent = OperationDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for PluginVersionOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),

    #[error("版本号已经存在 {0:?}")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x000B,
        http_code = "HttpCode::CONFLICT"
    ))]
    ConflictVersion(Version),
    #[error("版本信息不存在 {0:?}")]
    #[status_err(err(err_code = 0x0004, prefix = "ErrPrefix::NOT_FOUND",))]
    VersionNotFind(Version),
    #[error("暂没有版本信息")]
    #[status_err(err(err_code = 0x0005, prefix = "ErrPrefix::NOT_FOUND",))]
    VersionInfoNoExist,
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> OperationDatabaseOperate<'db, Conn> {
    pub fn plugin_version(&self) -> PluginVersionOperate<'_, Conn> {
        self.child()
    }
}
