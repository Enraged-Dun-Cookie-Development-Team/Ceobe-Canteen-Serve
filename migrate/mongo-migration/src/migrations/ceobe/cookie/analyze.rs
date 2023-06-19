use async_trait::async_trait;
use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::cookie::analyze::models::AnalyzeModel;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

const SOURCE_CONFIG_ID_IDX: &str = "source_config_id_idx";
const KEYWORD_IDX: &str = "keyword_idx";
const TAG_IDX: &str = "tag_idx";

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    type Model = AnalyzeModel;

    fn name(&self) -> &'static str { "ceobe_cookie_analyze" }

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
                            .name(SOURCE_CONFIG_ID_IDX.to_owned())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;
        collection
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "keywords.$**": 1i32,
                    })
                    .options(
                        IndexOptions::builder()
                            .name(KEYWORD_IDX.to_owned())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;
        collection
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "tags.$**": 1i32,
                    })
                    .options(
                        IndexOptions::builder()
                            .name(TAG_IDX.to_owned())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;

        Ok(())
    }
}
