mod update;

pub struct PluginDbOperation;

#[derive(Debug, thiserror::Error)]
pub enum OperateError {
    #[error("数据库查询异常")]
    Db(#[from] MongoDbError),

    #[error("版本号已经存在 {0:?}")]
    ConflictVersion(Version),
}

use mongo_connection::{get_mongo_collection, CollectionGuard, MongoDbError};

use super::models::{PluginVersion, PluginVersionChecked, Version};

fn get_plugin_version_collection(
) -> Result<CollectionGuard<PluginVersion>, MongoDbError> {
    get_mongo_collection()
}
