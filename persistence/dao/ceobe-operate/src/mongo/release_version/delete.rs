use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::operation::version::models::{
        ReleasePlatform, ReleaseVersion, Version,
    },
    mongodb::bson::{doc, to_bson},
};

use super::Result;
impl<'db, Conn> super::ReleaseVersionDelete<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ReleaseVersion>,
{
    /// 撤回一个已经发布的版本
    pub async fn yank(
        &'db self, platform: &ReleasePlatform, version: &Version,
    ) -> Result<()> {
        let collection = self.get_collection()?;

        collection
            .doing(|collection| {
                collection.find_one_and_update(
                    doc! {
                        "platform": to_bson(platform)?,
                        "version": to_bson(version)?,
                        "yanked":false
                    },
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
