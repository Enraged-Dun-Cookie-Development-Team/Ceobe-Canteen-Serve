
use sea_orm_migration::prelude::*;

use super::CeobeOperationToolLink;


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20231018_162927_ceobe_operation_friend_link_create" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(CeobeOperationToolLink::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CeobeOperationToolLink::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationToolLink::Nickname)
                    .string_len(32)
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationToolLink::Avatar)
                    .string_len(256)
                    .not_null(),
            );
        table.character_set("utf8mb4").collate("utf8mb4_general_ci");
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(CeobeOperationToolLink::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
