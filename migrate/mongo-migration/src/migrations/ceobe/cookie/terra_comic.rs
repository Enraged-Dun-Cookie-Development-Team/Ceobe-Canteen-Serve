use async_trait::async_trait;
use mongo_migrate_util::{CollectManage, MigrationTrait};
use mongo_models::ceobe::cookie::terra_comic::models::TerraComicModel;

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    type Model = TerraComicModel;

    fn name(&self) -> &'static str { "ceobe_cookie_terra_comic" }

    async fn migrate(
        &self, _mut_collection: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error> {
        Ok(())
    }
}
