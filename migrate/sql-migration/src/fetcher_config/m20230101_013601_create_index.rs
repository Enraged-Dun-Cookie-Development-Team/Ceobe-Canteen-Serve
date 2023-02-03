use sea_orm_migration::prelude::*;

use super::FetcherConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20230101_013601_fetcher_config_create_index" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut ci = sea_query::Index::create();
        ci.table(FetcherConfig::Table)
            .col(FetcherConfig::LiveNumber)
            .col(FetcherConfig::DatasourceId)
            .name("single-datasource")
            .unique();

        manager.create_index(ci).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut di = sea_query::Index::drop();
        di.table(FetcherConfig::Table).name("single-datasource");

        manager.drop_index(di).await?;
        Ok(())
    }
}
