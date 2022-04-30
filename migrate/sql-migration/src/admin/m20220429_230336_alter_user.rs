use sea_schema::migration::prelude::*;

use super::m20220429_142528_alter_user::User;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220429_230336_alter_user" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(User::Table).modify_column(
            ColumnDef::new(User::NumPwdChange)
                .not_null()
                .unsigned()
                .default(0),
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(User::Table).modify_column(
            ColumnDef::new(User::NumPwdChange).unsigned().default(0),
        );
        manager.alter_table(al).await?;

        Ok(())
    }
}
