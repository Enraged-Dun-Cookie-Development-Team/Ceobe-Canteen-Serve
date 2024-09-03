use sea_orm_migration::{
    prelude::*,
    sea_orm::{DeriveActiveEnum, EnumIter},
};
use sql_models::admin_user::{AuthLevel, Column::Auth};

use super::m20220722_082735_change_user_table_name::AdminUser;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20240903_224011_admin_alter_auth" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(AdminUser::Table).modify_column(
            ColumnDef::new_with_type(Auth, AuthLevel::column_type())
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 这边回滚需要删除一下authlevel中的Outsourcing
        let mut al = sea_query::Table::alter();
        al.table(AdminUser::Table).modify_column(
            ColumnDef::new_with_type(Auth, OldAuthLevel::column_type())
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "auth")]
pub enum OldAuthLevel {
    #[sea_orm(string_value = "chef")]
    Chef,
    #[sea_orm(string_value = "cooker")]
    Cooker,
    #[sea_orm(string_value = "architect")]
    Architect,
    #[sea_orm(string_value = "outsourcing")]
    Outsourcing,
}
