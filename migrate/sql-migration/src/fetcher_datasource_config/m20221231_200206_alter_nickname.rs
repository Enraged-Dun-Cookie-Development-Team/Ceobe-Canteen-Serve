use sea_orm_migration::prelude::*;

use super::FetcherDatasourceConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221231_200206_fetcher_datasource_config_alter_nickname"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table).modify_column(
            ColumnDef::new(FetcherDatasourceConfig::Nickname)
                .string_len(32)
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table).modify_column(
            ColumnDef::new(FetcherDatasourceConfig::Nickname)
                .string_len(16)
                .unique_key()
                .not_null(),
        );
        manager.alter_table(al).await?;

        Ok(())
    }
}
