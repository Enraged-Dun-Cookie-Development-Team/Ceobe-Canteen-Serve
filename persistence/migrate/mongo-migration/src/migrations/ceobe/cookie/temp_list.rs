use async_trait::async_trait;
use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::cookie::temp_list::models::TempListModel;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

const SOURCE_ID_IDX: &str = "source_id_idx";

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    type Model = TempListModel;

    fn name(&self) -> &'static str { "ceobe_cookie_temp_list" }

    async fn migrate(
        &self, mut collection: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error> {
        collection
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "source_config_id": 1i32,
                    })
                    .options(
                        IndexOptions::builder()
                            .name(SOURCE_ID_IDX.to_owned())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;

        Ok(())
    }
}
