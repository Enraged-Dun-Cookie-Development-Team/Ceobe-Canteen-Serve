const MANSION_IDX_NAME: &str = "mansion_idx";
const MANSION_CREATE_TIME_IDX_NAME: &str = "mansion_create_time_idx";

use mongo_migrate_util::MigrationTrait;
use mongo_models::bakery::mansion::preludes::*;
use mongodb::{
    bson::doc, options::IndexOptions, ClientSession, Collection, IndexModel,
};

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    type Model = ModelMansion;

    fn name(&self) -> &'static str { "bakery_mansion" }

    async fn migrate(
        &self, mansion: &Collection<Self::Model>, session: &mut ClientSession,
    ) -> Result<(), mongodb::error::Error> {
        let all_idx = mansion
            .list_index_names_with_session(session)
            .await
            .unwrap_or_default();
        log::info!(" all idx {:?}", all_idx);

        log::info!("注册 {} Index", MANSION_IDX_NAME);
        if !all_idx.contains(&MANSION_IDX_NAME.to_string()) {
            mansion
                .create_index_with_session(
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
                    session,
                )
                .await?;
        }
        log::info!("注册 {} Index", MANSION_IDX_NAME);
        if !all_idx.contains(&MANSION_CREATE_TIME_IDX_NAME.to_string()) {
            mansion
                .create_index_with_session(
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
                    session,
                )
                .await?;
        }

        Ok(())
    }
}
