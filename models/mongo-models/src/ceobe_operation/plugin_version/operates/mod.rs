pub mod retrieve;
mod update;
mod verify;

use mongo_connection::MongoDbError;
use status_err::{ErrPrefix, HttpCode, StatusErr};

use super::models::{PluginVersionChecked, Version};

pub struct PluginDbOperation;

#[derive(Debug, thiserror::Error, StatusErr)]
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
