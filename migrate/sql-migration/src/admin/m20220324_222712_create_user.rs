use sea_orm_migration::prelude::*;
use sql_models::admin_user::{
    AuthLevel, Column::*,
};
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220324_222712_create_user" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(User::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Username)
                    .string_len(16)
                    .unique_key()
                    .not_null(),
            )
            .col(ColumnDef::new(Password).char_len(64).not_null())
            .col(
                ColumnDef::new_with_type(Auth, AuthLevel::column_type())
                    .not_null(),
            );
        manager.create_table(table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(User::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}

#[derive(Debug, Iden)]
pub(super) enum User {
    Table,
}
