use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::operation::version::models::{
        DownloadSourceItem, ReleasePlatform, ReleaseVersion, Version,
    },
};

use crate::release_version::common::{
    generate_release_version_filter, generate_set_document,
};

impl<'db, C> super::ReleaseVersionUpdate<'db, C>
where
    C: MongoDbCollectionTrait<'db, ReleaseVersion>,
{
    pub async fn description(
        &'db self, version: Version, platform: ReleasePlatform,
        new_description: impl Into<Option<String>>,
    ) -> super::Result<()> {
        let collection = self.get_collection()?;

        let filter = generate_release_version_filter(&version, &platform)?;
        let update =
            generate_set_document("description", new_description.into())?;

        collection
            .doing(|collection| collection.update_one(filter, update, None))
            .await?;

        Ok(())
    }

    pub async fn download_resource(
        &'db self, version: Version, release_platform: ReleasePlatform,
        resource: Vec<DownloadSourceItem>,
    ) -> super::Result<()> {
        let collect = self.get_collection()?;

        let filter =
            generate_release_version_filter(&version, &release_platform)?;
        let update = generate_set_document("download_source", resource)?;

        collect
            .doing(|collect| collect.update_one(filter, update, None))
            .await?;

        Ok(())
    }
}
