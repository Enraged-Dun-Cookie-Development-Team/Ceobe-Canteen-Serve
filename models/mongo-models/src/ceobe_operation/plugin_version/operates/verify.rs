use mongo_connection::CollectionGuard;
use mongodb::bson::doc;

use super::{OperateError, OperateResult, PluginDbOperation, Version};
use crate::ceobe_operation::plugin_version::models::PluginVersion;

impl PluginDbOperation {
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
        if collect
            .doing(|collect| collect.count_documents(filter, None))
            .await?
            > 0
        {
            Err(OperateError::ConflictVersion(version))?;
        }

        Ok(())
    }
}
