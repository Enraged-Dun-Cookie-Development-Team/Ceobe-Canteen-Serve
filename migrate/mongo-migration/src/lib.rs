mod migrations;

use async_trait::async_trait;
use mongo_migrate_util::{Manager, MigratorTrait};
use mongo_models::bakery::mansion::operate::MongoErr;
pub use mongo_models::{self, mongo_connection};
pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    async fn migrating(&self, manage: &mut Manager<'_>) -> Result<(), MongoErr> {
        manage
            .append(migrations::bakery::mansion::Migration)
            .await?
            ;

        Ok(())
    }
}
