use async_trait::async_trait;
use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::operation::version::models::ReleaseVersion;
use mongodb::{bson::doc, error::Error, options::IndexOptions, IndexModel};

const UNIQUE_KEY: &str = "unique_version_platform";

pub struct Migration;
#[async_trait]
impl MigrationTrait for Migration {
    type Model = ReleaseVersion;

    fn name(&self) -> &'static str { "ceobe_operation_release_version" }

    async fn migrate(
        &self, mut collection: CollectManage<Self>,
    ) -> Result<(), Error> {
        collection
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "version": 1i32,
                        "platform": 1i32
                    })
                    .options(
                        IndexOptions::builder()
                            .unique(true)
                            .name(UNIQUE_KEY.to_owned())
                            .build(),
                    )
                    .build(),
                None
            )
            .await?;
        Ok(())
    }
}
