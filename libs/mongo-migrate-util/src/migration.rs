use mongodb::options::CreateCollectionOptions;
use serde::{Deserialize, Serialize};

use crate::CollectManage;

#[async_trait::async_trait]
pub trait MigrationTrait: Sync + Send + Sized {
    /// the model that migrate
    type Model: Serialize + for<'de> Deserialize<'de> + 'static;

    /// the model name, using for searching
    fn name(&self) -> &'static str;

    fn create_options(&self) -> Option<CreateCollectionOptions> { None }

    /// doing migration with session
    async fn migrate(
        &self, collection: CollectManage<Self>,
    ) -> Result<(), mongodb::error::Error>;
}
