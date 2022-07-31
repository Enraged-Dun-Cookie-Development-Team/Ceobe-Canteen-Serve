use mongodb::{bson::doc, options::CountOptions};

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
        let filter = doc! {
            "version" : [
                plugin_version.version.0,
                plugin_version.version.1,
                plugin_version.version.2,
            ]

        };
        // checker version exist
        let count = db
            .doing(|collect| {
                collect.count_documents(
                    filter,
                    CountOptions::builder().limit(1).build(),
                )
            })
            .await?;

        if count > 0 {
            Err(OperateError::ConflictVersion(
                plugin_version.version.clone(),
            ))?;
        }

        // update

        db.doing(|collect| collect.insert_one(plugin_version, None))
            .await?;

        Ok(())
    }
}
