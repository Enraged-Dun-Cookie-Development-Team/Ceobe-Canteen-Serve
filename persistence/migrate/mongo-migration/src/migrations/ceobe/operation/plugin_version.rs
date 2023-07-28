use async_trait::async_trait;
use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::operation::plugin_version::PluginVersion;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

const UNIQUE_VERSION_IDX: &str = "unique_version_idx";

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    type Model = PluginVersion;

    fn name(&self) -> &'static str { "ceobe_operation_plugin_version" }

    async fn migrate(
        &self, mut collection: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error> {
        collection
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "version.major": 1i32,
                        "version.minor": 1i32,
                        "version.security": 1i32,
                    })
                    .options(
                        IndexOptions::builder()
                            .unique(true)
                            .name(UNIQUE_VERSION_IDX.to_owned())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;

        Ok(())
    }
}
