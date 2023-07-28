use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::bakery::mansion::preludes::*;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use tracing::{info, instrument};

const MANSION_IDX_NAME: &str = "mansion_idx";
const MANSION_CREATE_TIME_IDX_NAME: &str = "mansion_create_time_idx";

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    type Model = ModelMansion;

    fn name(&self) -> &'static str { "bakery_mansion" }

    #[instrument(name = "migrate of mansion", skip_all)]
    async fn migrate(
        &self, mut mansion: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error> {
        info!(mansion.migrate.createIndex.name = MANSION_IDX_NAME);
        mansion
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "id.main_id":1i32,
                        "id.minor_id":1i32,
                    })
                    .options(
                        IndexOptions::builder()
                            .unique(true)
                            .name(MANSION_IDX_NAME.to_string())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;

        info!(
            mansion.migrate.createIndex.name = MANSION_CREATE_TIME_IDX_NAME
        );
        mansion
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "create_time":1i32
                    })
                    .options(
                        IndexOptions::builder()
                            .name(MANSION_CREATE_TIME_IDX_NAME.to_string())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;

        Ok(())
    }
}
