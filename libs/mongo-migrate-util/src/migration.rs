use mongodb::{options::CreateCollectionOptions, ClientSession, Collection};
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait MigrationTrait: Sync + Send {
    /// the model that migrate
    type Model: Serialize + for<'de> Deserialize<'de> + 'static;

    /// the model name, using for searching
    fn name(&self) -> &'static str;

    fn create_options(&self) -> Option<CreateCollectionOptions> { None }

    /// doing migration with session
    async fn migrate(
        &self, collection: &Collection<Self::Model>,
        session: &mut ClientSession,
    ) -> Result<(), mongodb::error::Error>;
}
