use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongodb::{bson::doc, options::FindOneOptions},
};
use tracing::{info, instrument};

use super::{
    OperateError, OperateResult, PluginVersion, PluginVersionOperate, Version,
};

impl<'db, Conn> PluginVersionOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, PluginVersion>,
{
    #[instrument(skip(self), ret)]
    pub async fn get_info_by_version(
        &'db self, version: Version,
    ) -> OperateResult<PluginVersion> {
        info!(plugin.version = %version);
        let collection = self.get_collection()?;
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

    #[instrument(skip(self), ret)]
    pub async fn get_newest_info(&'db self) -> OperateResult<PluginVersion> {
        info!(plugin.version = "latest");
        let collection = self.get_collection()?;
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
