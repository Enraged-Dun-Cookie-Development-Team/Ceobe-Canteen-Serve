use mongo_connection::MongoDbCollectionTrait;
use mongodb::{bson::doc, options::FindOneOptions};

use super::{OperateError, OperateResult, PluginDbOperation};
use crate::ceobe_operation::plugin_version::models::{
    PluginVersion, Version,
};

impl PluginDbOperation {
    pub async fn get_plugin_version_info_by_version<'db, D>(
        db: &'db D, version: Version,
    ) -> OperateResult<PluginVersion>
    where
        D: MongoDbCollectionTrait<'db, PluginVersion>,
    {
        let collection = db.get_collection()?;
        let filter = doc! {
            "version.major": version.major,
            "version.minor": version.minor,
            "version.security": version.security
        };
        collection
            .doing(|collection| collection.find_one(filter, None))
            .await?
            .ok_or_else(|| OperateError::VersionNotFind(version))
    }

    pub async fn get_newest_plugin_version_info<'db, D>(
        db: &'db D,
    ) -> OperateResult<PluginVersion>
    where
        D: MongoDbCollectionTrait<'db, PluginVersion>,
    {
        let collection = db.get_collection()?;
        collection
            .doing(|collection|
                collection.find_one(
                    None,
                    FindOneOptions::builder()
                    .sort(doc! {"version.major":-1, "version.minor":-1, "version.security":-1})
                    .build()
                )
            )
            .await?
            .ok_or(OperateError::VersionInfoNoExist)
    }
}
