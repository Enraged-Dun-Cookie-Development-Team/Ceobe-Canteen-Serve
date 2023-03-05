mod migrations;

use async_trait::async_trait;
use mongo_migrate_util::{Manager, MigratorTrait};
pub use mongo_models::{self, mongo_connection};
use mongodb::error::Error;
pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    async fn migrating(&self, manage: &mut Manager<'_>) -> Result<(), Error> {
        manage
            .append(migrations::bakery::mansion::Migration)
            .await?
            .append(migrations::ceobe_operation::plugin_version::Migration)
            .await?
            .append(migrations::ceobe::user_property::Migration)
            .await?;

        Ok(())
    }
}
