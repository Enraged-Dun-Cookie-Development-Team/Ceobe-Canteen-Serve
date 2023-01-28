use db_ops_prelude::database_operates::NoConnect;
use db_ops_prelude::mongo_connection::CollectionGuard;
use db_ops_prelude::mongodb::bson::doc;
use tracing::warn;

use super::PluginVersionOperate;
use super::{OperateError, OperateResult};
use super::{PluginVersion, Version};
impl PluginVersionOperate<'_, NoConnect> {
    pub async fn verify_version(
        version: Version, collect: &CollectionGuard<PluginVersion>,
    ) -> OperateResult<()> {
        // version can not be the same even is delete
        let filter = doc! {
            "version" : {
                "major": version.major,
                "minor": version.minor,
                "security": version.security
            }
        };
        // checker version exist
        let 0  =  collect
            .doing(|collect| collect.count_documents(filter, None))
            .await? else {
            warn!(pluginVersion.version = %version, pluginVersion.exist = true);
            return Err(OperateError::ConflictVersion(version))
        };

        Ok(())
    }
}
