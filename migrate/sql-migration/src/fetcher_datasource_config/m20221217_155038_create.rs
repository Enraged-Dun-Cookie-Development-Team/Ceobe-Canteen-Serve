use sea_orm_migration::prelude::*;

use super::FetcherDatasouceConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221217_155038_fetcher_datasource_config_create"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(FetcherDatasouceConfig::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FetcherDatasouceConfig::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasouceConfig::Platform)
                    .string_len(64)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasouceConfig::Datasource)
                    .string_len(64)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasouceConfig::Nickname)
                    .string_len(16)
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasouceConfig::Avatar)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasouceConfig::Config)
                    .text()
                    .not_null()
                    .default("{}"),
            )
            .col(
                ColumnDef::new(FetcherDatasouceConfig::UniqueId)
                    .uuid()
                    .not_null()
            );
        table.index(
            Index::create()
            .col(FetcherDatasouceConfig::Platform)
            .col(FetcherDatasouceConfig::Nickname)
            .col(FetcherDatasouceConfig::Config)
            .name("single-datasource")
            .unique(),
        );
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(FetcherDatasouceConfig::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
