use sea_orm_migration::prelude::*;

use super::FetcherDatasourceConfig;

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
            .table(FetcherDatasourceConfig::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FetcherDatasourceConfig::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceConfig::Platform)
                    .string_len(64)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceConfig::Datasource)
                    .string_len(64)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceConfig::Nickname)
                    .string_len(16)
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceConfig::Avatar)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceConfig::Config)
                    .text()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceConfig::UniqueId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceConfig::DbUniqueKey)
                    .string_len(64)
                    .not_null(),
            );
        table.index(
            Index::create()
                .col(FetcherDatasourceConfig::Datasource)
                .col(FetcherDatasourceConfig::DbUniqueKey)
                .name("single-datasource")
                .unique(),
        );
        table.character_set("utf8mb4").collate("utf8mb4_general_ci");
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(FetcherDatasourceConfig::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
