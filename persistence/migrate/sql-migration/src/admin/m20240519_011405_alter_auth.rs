
use sea_orm_migration::prelude::*;

use super::m20220324_222712_create_user::User;
use sql_models::admin_user::{AuthLevel, Column::Auth};


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20240519_011405_admin_alter_auth" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(User::Table).modify_column(
            ColumnDef::new_with_type(Auth, AuthLevel::column_type())
                    .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 这边回滚需要删除一下authlevel中的Outsourcing
        let mut al = sea_query::Table::alter();
        al.table(User::Table).modify_column(
            ColumnDef::new_with_type(Auth, AuthLevel::column_type())
                    .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }
}
