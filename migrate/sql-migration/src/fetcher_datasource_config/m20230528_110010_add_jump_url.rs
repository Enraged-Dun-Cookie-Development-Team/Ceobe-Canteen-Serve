
use sea_orm_migration::prelude::*;

use super::FetcherDatasourceConfig;


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20230528_110010_fetcher_datasource_config_add_jump_url" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table).add_column(
            ColumnDef::new(FetcherDatasourceConfig::JumpUrl)
                .string_len(256)
        );
        manager.alter_table(al).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table)
            .drop_column(FetcherDatasourceConfig::JumpUrl);
        manager.alter_table(al).await?;
        Ok(())
    }
}
