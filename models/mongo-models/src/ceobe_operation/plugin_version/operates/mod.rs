pub mod retrieve;
mod update;
mod verify;

use mongo_connection::{get_mongo_collection, CollectionGuard, MongoDbError};
use status_err::{ErrPrefix, HttpCode, StatusErr};

use super::models::{PluginVersion, PluginVersionChecked, Version};

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
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x000D,
    ))]
    VersionNotFind(Version),
    #[error("暂没有版本信息")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x000E,
    ))]
    VersionInfoNoExist,
}

fn get_plugin_version_collection(
) -> Result<CollectionGuard<PluginVersion>, MongoDbError> {
    get_mongo_collection()
}

type OperateResult<T> = Result<T, OperateError>;