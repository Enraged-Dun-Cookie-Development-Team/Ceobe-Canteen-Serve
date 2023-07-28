use async_trait::async_trait;
use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::cookie::terra_comic::models::TerraComicModel;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

pub struct Migration;

const UNIQUE_CID_IDX: &str = "unique_cid_idx";

#[async_trait]
impl MigrationTrait for Migration {
    type Model = TerraComicModel;

    fn name(&self) -> &'static str { "ceobe_cookie_terra_comic" }

    async fn migrate(
        &self, mut collection: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error> {
        collection
            .create_idx_if_not_exist(
                IndexModel::builder()
                    .keys(doc! {
                        "cid": 1i32,
                    })
                    .options(
                        IndexOptions::builder()
                            .unique(true)
                            .name(UNIQUE_CID_IDX.to_owned())
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;
        Ok(())
    }
}
