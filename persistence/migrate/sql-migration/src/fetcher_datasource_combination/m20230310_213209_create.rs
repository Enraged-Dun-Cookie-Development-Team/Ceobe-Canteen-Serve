use sea_orm_migration::prelude::*;

use super::FetcherDatasourceCombination;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230310_213209_fetcher_datasource_combination_create"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(FetcherDatasourceCombination::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FetcherDatasourceCombination::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceCombination::CombinationId)
                    .string_len(64)
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceCombination::Bitmap1)
                    .bit(Some(64))
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceCombination::Bitmap2)
                    .bit(Some(64))
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceCombination::Bitmap3)
                    .bit(Some(64))
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceCombination::Bitmap4)
                    .bit(Some(64))
                    .not_null(),
            )
            .col(
                ColumnDef::new(FetcherDatasourceCombination::LastAccessTime)
                    .date_time()
                    .not_null(),
            );
        table
            .index(
                Index::create()
                    .col(FetcherDatasourceCombination::Bitmap1)
                    .name("bitmap_1"),
            )
            .index(
                Index::create()
                    .col(FetcherDatasourceCombination::Bitmap2)
                    .name("bitmap_2"),
            )
            .index(
                Index::create()
                    .col(FetcherDatasourceCombination::Bitmap3)
                    .name("bitmap_3"),
            )
            .index(
                Index::create()
                    .col(FetcherDatasourceCombination::Bitmap4)
                    .name("bitmap_4"),
            );
        table.character_set("utf8mb4").collate("utf8mb4_general_ci");
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(FetcherDatasourceCombination::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
