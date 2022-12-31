
use sea_orm_migration::prelude::*;

use super::FetcherConfig;


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20221231_150609_fetcher_config_alter_interval" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherConfig::Table).modify_column(
            ColumnDef::new(FetcherConfig::Interval).big_unsigned()
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherConfig::Table).modify_column(
            ColumnDef::new(FetcherConfig::Interval).integer()
        );
        manager.alter_table(al).await?;

        Ok(())
    }
}
