
use sea_orm_migration::prelude::*;

use super::FetcherPlatformConfig;




pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20221217_155027_fetcher_platform_config_create" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table.table(FetcherPlatformConfig::Table).if_not_exists()
        .col(
            ColumnDef::new(FetcherPlatformConfig::Id)
                .integer()
                .auto_increment()
                .primary_key()
                .not_null(),
        )
        .col(
            ColumnDef::new(FetcherPlatformConfig::TypeId)
            .string_len(64)
            .unique_key()
            .not_null(),
        )
        .col(
            ColumnDef::new(FetcherPlatformConfig::PlatformName)
            .string_len(16)
            .not_null(),
        )
        .col(
            ColumnDef::new(FetcherPlatformConfig::MinRequestInterval)
            .integer()
            .not_null(),
        );
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(FetcherPlatformConfig::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
