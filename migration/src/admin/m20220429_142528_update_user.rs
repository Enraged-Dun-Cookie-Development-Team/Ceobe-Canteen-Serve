use sea_schema::migration::prelude::*;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220429_142528_update_user" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(User::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(User::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(User::Username)
                    .string_len(16)
                    .unique_key()
                    .not_null(),
            )
            .col(ColumnDef::new(User::Password).char_len(64).not_null())
            .col(
                ColumnDef::new(User::Auth)
                    .enumeration("auth", ["chef", "cooker", "architect"])
                    .not_null(),
            )
            .col(ColumnDef::new(User::NumPwdChange).unsigned().default(0));
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
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Auth,
    NumPwdChange
}
