
use sea_orm_migration::prelude::*;

use super::FetcherDatasourceConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20231115_135841_fetcher_datasource_config_add_visual" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table).add_column(
            ColumnDef::new(FetcherDatasourceConfig::Visual).boolean().not_null().default(true),
        );
        manager.alter_table(al).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table)
            .drop_column(FetcherDatasourceConfig::Visual);
        manager.alter_table(al).await?;
        Ok(())
    }
}
