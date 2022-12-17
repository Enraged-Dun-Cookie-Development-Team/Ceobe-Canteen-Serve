
use sea_orm_migration::prelude::*;

use super::FetcherConfig;


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20221217_155140_fetcher_config_create" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table.table(FetcherConfig::Table).if_not_exists()
        .col(
            ColumnDef::new(FetcherConfig::Id)
                .integer()
                .auto_increment()
                .primary_key()
                .not_null(),
        )
        .col(
            ColumnDef::new(FetcherConfig::LiveNumber)
                .integer()
                .not_null()
        )
        .col(
            ColumnDef::new(FetcherConfig::FetcherCount)
                .integer()
                .not_null()
        )
        .col(
            ColumnDef::new(FetcherConfig::GroupName)
                .string_len(16)
                .not_null()
        )
        .col(
            ColumnDef::new(FetcherConfig::Platform)
                .string_len(64)
                .not_null()
        )
        .col(
            ColumnDef::new(FetcherConfig::DatasourceId)
            .integer()
                .not_null()
        )
        .col(
            ColumnDef::new(FetcherConfig::Interval)
            .integer()
        )
        .col(
            ColumnDef::new(FetcherConfig::IntervalByTimeRange)
            .text()
            .not_null()
            .default("[]")
        );
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(FetcherConfig::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
