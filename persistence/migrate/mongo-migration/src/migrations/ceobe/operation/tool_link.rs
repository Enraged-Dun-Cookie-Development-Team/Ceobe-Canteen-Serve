use async_trait::async_trait;
use mongodb::{bson::doc, IndexModel, options::IndexOptions};

use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::operation::tool_link::models::ToolLink;

const UNIQUE_VERSION_IDX: &str = "unique_id_idx";

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    type Model = ToolLink;

    fn name(&self) -> &'static str { "ceobe_operation_tool_link" }

    async fn migrate(
        &self, mut collection: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error> {
        collection
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "id":1i32
                    })
                    .options(
                        IndexOptions::builder()
                            .name(UNIQUE_VERSION_IDX.to_owned())
                            .unique(true)
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;

        Ok(())
    }
}
