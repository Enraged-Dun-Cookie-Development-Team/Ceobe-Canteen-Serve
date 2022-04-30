use super::manager::Manager;

#[async_trait::async_trait]
pub trait MigrationTrait :Sync+Send{
    async fn migrate(
        &self, manager: &Manager,
    ) -> Result<(), mongodb::error::Error>;
}
