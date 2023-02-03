use sea_orm_migration::prelude::*;

use super::FetcherGlobalConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20221217_155049_fetcher_global_config_create" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(FetcherGlobalConfig::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FetcherGlobalConfig::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherGlobalConfig::Key)
                    .string_len(64)
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherGlobalConfig::Value)
                    .string_len(64)
                    .not_null(),
            );
        table.character_set("utf8mb4").collate("utf8mb4_general_ci");
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(FetcherGlobalConfig::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
