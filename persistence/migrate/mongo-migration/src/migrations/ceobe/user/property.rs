use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::user_property::models::UserPropertyModel;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use tracing::{info, instrument};

const MOB_IDX_NAME: &str = "mob_idx";

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    type Model = UserPropertyModel;

    fn name(&self) -> &'static str { "ceobe_user_property" }

    #[instrument(name = "migrate of user", skip_all)]
    async fn migrate(
        &self, mut user: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error> {
        info!(user.migrate.createIndex.name = MOB_IDX_NAME);
        user.create_idx_if_not_exist(
            IndexModel::builder()
                .keys(doc! {
                    "mob_id":1i32
                })
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .name(MOB_IDX_NAME.to_string())
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

        Ok(())
    }
}
