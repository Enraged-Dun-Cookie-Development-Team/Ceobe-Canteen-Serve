use async_trait::async_trait;
use mongo_migrate_util::{Manager, MigratorTrait};
use mongodb::error::Error;

mod migrations;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    async fn migrating(&self, manage: &mut Manager<'_>) -> Result<(), Error> {
        manage
            .append(migrations::bakery::mansion::Migration)
            .await?
            .append(migrations::ceobe::operation::plugin_version::Migration)
            .await?
            .append(migrations::ceobe::user::property::Migration)
            .await?
            .append(migrations::ceobe::cookie::temp_list::Migration)
            .await?
            .append(migrations::ceobe::cookie::analyze::Migration)
            .await?
            .append(migrations::ceobe::cookie::raw::Migration)
            .await?
            .append(migrations::ceobe::cookie::terra_comic::Migration)
            .await?;

        Ok(())
    }
}
