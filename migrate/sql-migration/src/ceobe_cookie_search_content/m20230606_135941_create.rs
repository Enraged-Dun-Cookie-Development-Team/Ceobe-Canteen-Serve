use sea_orm_migration::prelude::*;

use super::CeobeCookieSearchContent;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230606_135941_ceobe_cookie_search_content_create"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(CeobeCookieSearchContent::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CeobeCookieSearchContent::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeCookieSearchContent::ObjectId)
                    .char_len(24)
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeCookieSearchContent::SourceConfigId)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeCookieSearchContent::Content)
                    .text()
                    .not_null(),
            );
        table.index(
            Index::create()
                .col(CeobeCookieSearchContent::Content)
                .full_text()
                .name("content"),
        );
        table.character_set("utf8mb4").collate("utf8mb4_general_ci");
        table.engine("Mroonga");
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(CeobeCookieSearchContent::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
