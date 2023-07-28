use sea_orm_migration::prelude::*;

use super::FetcherConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221231_205557_fetcher_config_alter_group_name"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherConfig::Table).modify_column(
            ColumnDef::new(FetcherConfig::GroupName)
                .string_len(32)
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherConfig::Table).modify_column(
            ColumnDef::new(FetcherConfig::GroupName)
                .string_len(16)
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }
}
