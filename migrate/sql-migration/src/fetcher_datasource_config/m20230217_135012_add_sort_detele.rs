use sea_orm_migration::prelude::*;
use sql_models::get_zero_data_time;

use super::FetcherDatasourceConfig;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230217_135012_fetcher_datasource_config_add_sort_detele"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table).add_column(
            ColumnDef::new(FetcherDatasourceConfig::DeleteAt)
                .date_time()
                .not_null()
                .default(Value::ChronoDateTime(
                    Box::new(get_zero_data_time()).into(),
                )),
        );
        manager.alter_table(al).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(FetcherDatasourceConfig::Table)
            .drop_column(FetcherDatasourceConfig::DeleteAt);
        let mut delete = sea_query::DeleteStatement::new();
        delete.from_table(FetcherDatasourceConfig::Table).and_where(
            Expr::col(FetcherDatasourceConfig::DeleteAt).ne(get_zero_data_time())
            );
        manager.exec_stmt(delete).await?;
        manager.alter_table(al).await?;
        Ok(())
    }
}
