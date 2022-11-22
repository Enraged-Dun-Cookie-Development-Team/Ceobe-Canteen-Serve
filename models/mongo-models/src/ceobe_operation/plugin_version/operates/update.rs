use mongo_connection::MongoDbCollectionTrait;
use tracing::{info, instrument};

use super::{OperateResult, PluginDbOperation, PluginVersionChecked};
use crate::{ceobe_operation::plugin_version::PluginVersion, RecordUnit};
impl PluginDbOperation {
    #[instrument(skip(db), name = "insert-new")]
    pub async fn update_new<'db>(
        db: &'db impl MongoDbCollectionTrait<'db, PluginVersion>,
        version: PluginVersionChecked,
    ) -> OperateResult<()> {
        info!(newPlugin.version = %version.version);
        let db = db.get_collection()?;

        let plugin_version = version.into_with_time_record(RecordUnit::new());

        // version can not be the same even is delete
        Self::verify_version(plugin_version.version, &db).await?;
        // update

        db.doing(|collect| collect.insert_one(plugin_version, None))
            .await?;

        Ok(())
    }
}
