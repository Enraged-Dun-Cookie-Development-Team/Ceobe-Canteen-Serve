use sea_orm_migration::prelude::*;

use super::FetcherPlatformConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221231_150609_fetcher_platform_config_alter_min_interval"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherPlatformConfig::Table).modify_column(
            ColumnDef::new(FetcherPlatformConfig::MinRequestInterval)
                .big_unsigned()
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherPlatformConfig::Table).modify_column(
            ColumnDef::new(FetcherPlatformConfig::MinRequestInterval)
                .integer()
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }
}
