use super::{
    get_plugin_version_collection, OperateError, PluginDbOperation,
    PluginVersionChecked,
};
use crate::RecordUnit;
impl PluginDbOperation {
    pub async fn update_new(
        version: PluginVersionChecked,
    ) -> Result<(), OperateError> {
        let db = get_plugin_version_collection()?;

        let plugin_version = version.into_with_time_record(RecordUnit::new());

        // version can not be the same even is delete
        Self::verify_version(plugin_version.version, &db).await?;
        // update

        db.doing(|collect| collect.insert_one(plugin_version, None))
            .await?;

        Ok(())
    }
}
