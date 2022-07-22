use sea_orm_migration::prelude::*;

use super::m20220324_222712_create_user::User;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220429_230336_change_user_table_name" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut rename = sea_query::Table::rename();
        rename.table(User::Table, AdminUser::Table);

        manager.exec_stmt(rename).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut rename = sea_query::Table::rename();
        rename.table(AdminUser::Table, User::Table);

        manager.exec_stmt(rename).await?;
        Ok(())
    }
}

#[derive(Debug, Iden)]
pub enum AdminUser {
    Table,
}
