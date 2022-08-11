use mongodb::{bson::doc, options::FindOneOptions};

use super::{
    get_plugin_version_collection, OperateError, OperateResult,
    PluginDbOperation,
};
use crate::ceobe_operation::plugin_version::models::{
    PluginVersion, Version,
};

impl PluginDbOperation {
    pub async fn get_plugin_version_info_by_version(
        version: Version,
    ) -> OperateResult<PluginVersion> {
        let collection = get_plugin_version_collection()?;
        let filter = doc! {
            "version.major": version.major,
            "version.minor": version.minor,
            "version.security": version.security
        };
        collection
            .doing(|collection| collection.find_one(filter, None))
            .await?
            .ok_or(OperateError::VersionNotFind(version))
    }

    pub async fn get_newest_plugin_version_info(
    ) -> OperateResult<PluginVersion> {
        let collection = get_plugin_version_collection()?;
        collection
            .doing(|collection|
                collection.find_one(
                    None,
                    FindOneOptions::builder()
                    .projection(doc! {"version":1i32})
                    .sort(doc! {"version.major":-1, "version.minor":-1, "version.security":-1})
                    .build()
                )
            )
            .await?
            .ok_or(OperateError::VersionInfoNoExist)
    }
}
