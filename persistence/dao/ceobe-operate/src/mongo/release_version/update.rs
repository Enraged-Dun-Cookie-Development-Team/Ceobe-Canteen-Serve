use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::operation::version::models::{
        DownloadSourceItem, ReleasePlatform, ReleaseVersion, Version,
    },
    mongodb::bson::{doc, to_bson},
};

use super::Result;
use crate::release_version::common::generate_release_version_filter;

impl<'db, C> super::ReleaseVersionUpdate<'db, C>
where
    C: MongoDbCollectionTrait<'db, ReleaseVersion>,
{
    pub async fn description_and_resource(
        &'db self, version: Version, platform: ReleasePlatform,
        new_description: impl Into<Option<String>>,
        resource: Vec<DownloadSourceItem>,
    ) -> Result<()> {
        let collection = self.get_collection()?;

        let filter = generate_release_version_filter(&version, &platform)?;
        let update = doc! {
            "$set":{
                "description": to_bson(&new_description.into())?,
                "download_source":to_bson(&resource)?
            }
        };

        collection
            .doing(|collection| collection.update_one(filter, update, None))
            .await?;

        Ok(())
    }

    /// 撤回一个已经发布的版本
    pub async fn yank(
        &'db self, platform: &ReleasePlatform, version: &Version,
    ) -> Result<()> {
        let collection = self.get_collection()?;
        let filter = doc! {
            "platform": to_bson(platform)?,
            "version": to_bson(version)?,
            "yanked":false
        };
        collection
            .doing(|collection| {
                collection.find_one_and_update(
                    filter,
                    doc! {
                        "$set":{
                            "yanked": true
                        }
                    },
                    None,
                )
            })
            .await?;

        Ok(())
    }
}
