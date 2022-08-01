mod update;
mod verify;

use mongo_connection::{get_mongo_collection, CollectionGuard, MongoDbError};
use status_err::{ErrPrefix, HttpCode, StatusErr};

use super::models::{PluginVersion, PluginVersionChecked, Version};

pub struct PluginDbOperation;

#[derive(Debug, thiserror::Error)]
pub enum OperateError {
    #[error("数据库查询异常")]
    Db(#[from] MongoDbError),

    #[error("版本号已经存在 {0:?}")]
    ConflictVersion(Version),
}

impl StatusErr for OperateError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            OperateError::Db(inner) => inner.prefix(),
            OperateError::ConflictVersion(_) => ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        match self {
            OperateError::Db(inner) => inner.code(),
            OperateError::ConflictVersion(_) => 0x000B,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            OperateError::ConflictVersion(_) => HttpCode::CONFLICT,
            _ => self.status().http_code(),
        }
    }
}

fn get_plugin_version_collection(
) -> Result<CollectionGuard<PluginVersion>, MongoDbError> {
    get_mongo_collection()
}
