use async_trait::async_trait;
use mongo_migrate_util::{Manager, MigrationTrait};
use mongo_models::ceobe_operation::plugin_version::models::PluginVersion;
use mongodb::{bson::doc, error, options::IndexOptions, IndexModel};

const UNIQUE_VERSION_IDX: &str = "unique_version_idx";

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn migrate(&self, manager: &Manager) -> Result<(), error::Error> {
        let plugin_version = manager
            .collection::<PluginVersion, _>("ceobe_operation_plugin_version");

        let exist_idx =
            plugin_version.list_index_names().await.unwrap_or_default();
        log::info!("All idx of {} : {:?}", plugin_version.name(), exist_idx);

        // adding unique index
        if !exist_idx.contains(&UNIQUE_VERSION_IDX.to_owned()) {
            log::info!(
                "idx {} not exist in {} ,create",
                UNIQUE_VERSION_IDX,
                plugin_version.name()
            );
            plugin_version
                .create_index(
                    IndexModel::builder()
                        .keys(doc! {
                            "version":1i32,
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
        }

        Ok(())
    }
}
