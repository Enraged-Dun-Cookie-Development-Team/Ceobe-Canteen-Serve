use sea_orm_migration::prelude::*;
use sql_models::admin_user::models::user::Column::NumPwdChange;

use super::m20220324_222712_create_user::User;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220429_142528_alter_user" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(User::Table)
            .add_column(ColumnDef::new(NumPwdChange).unsigned().default(0));
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(User::Table).drop_column(NumPwdChange);
        manager.alter_table(al).await?;

        Ok(())
    }
}
