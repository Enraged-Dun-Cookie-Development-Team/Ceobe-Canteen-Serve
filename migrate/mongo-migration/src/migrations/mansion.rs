const MANSION_IDX_NAME: &str = "mansion_idx";
const MANSION_CREATE_TIME_IDX_NAME: &str = "mansion_create_time_idx";

use mongo_connection::{Manager, MigrationTrait};
use mongo_models::mansion::preludes::*;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn migrate(
        &self, manager: &Manager,
    ) -> Result<(), mongodb::error::Error> {
        let mansion = manager.collection::<ModelMansion, _>("mansion-data");
        let all_idx = mansion.list_index_names().await.unwrap_or_default();
        log::info!(" all idx {:?}", all_idx);

        log::info!("注册 {} Index", MANSION_IDX_NAME);
        if !all_idx.contains(&MANSION_IDX_NAME.to_string()) {
            mansion
                .create_index(
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
        }
        log::info!("注册 {} Index", MANSION_IDX_NAME);
        if !all_idx.contains(&MANSION_CREATE_TIME_IDX_NAME.to_string()) {
            mansion
                .create_index(
                    IndexModel::builder()
                        .keys(doc! {
                            "create_time":1i32
                        })
                        .options(
                            IndexOptions::builder()
                                .name(
                                    MANSION_CREATE_TIME_IDX_NAME.to_string(),
                                )
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
