use sea_orm_migration::prelude::*;

use super::CeobeOperationToolLink;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240519_004238_ceobe_operation_tool_link_add_infos"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(CeobeOperationToolLink::Table)
            .add_column(
                ColumnDef::new(CeobeOperationToolLink::Slogan)
                    .string_len(16)
                    .not_null()
                    .default(""),
            )
            .add_column(
                ColumnDef::new(CeobeOperationToolLink::Description)
                    .string_len(64)
                    .not_null()
                    .default(""),
            )
            .add_column(
                ColumnDef::new(CeobeOperationToolLink::Tags)
                    .string_len(64)
                    .not_null()
                    .default("[]"),
            );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(CeobeOperationToolLink::Table)
            .drop_column(CeobeOperationToolLink::Slogan)
            .drop_column(CeobeOperationToolLink::Description)
            .drop_column(CeobeOperationToolLink::Tags);
        manager.alter_table(al).await?;

        Ok(())
    }
}
