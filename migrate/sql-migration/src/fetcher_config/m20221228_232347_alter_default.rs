
use sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::Keyword::Null;
use sea_orm_migration::prelude::SimpleExpr::Keyword;
use sea_orm_migration::sea_orm::{Statement, ConnectionTrait};
use super::FetcherConfig;


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20221228_232347_fetcher_config_alter_default" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let mut al = sea_query::Table::alter();
        al.table(FetcherConfig::Table)
            .modify_column(&mut ColumnDef::new(FetcherConfig::IntervalByTimeRange).text().null().default(Keyword(Null)));
        let update = Query::update().table(FetcherConfig::Table).value(FetcherConfig::IntervalByTimeRange, Keyword(Null)).and_where(Expr::col(FetcherConfig::IntervalByTimeRange).eq("[]")).to_owned();
        print!("{:?}", update.to_string(MysqlQueryBuilder));
        manager.alter_table(al).await?;
        manager.exec_stmt(update).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let update = Query::update().table(FetcherConfig::Table).value(FetcherConfig::IntervalByTimeRange, "[]").and_where(Expr::col(FetcherConfig::IntervalByTimeRange).eq(Keyword(Null))).to_owned();
        let mut al = sea_query::Table::alter();
        al.table(FetcherConfig::Table)
            .modify_column(&mut ColumnDef::new(FetcherConfig::IntervalByTimeRange).text().not_null().default("[]"));

        manager.exec_stmt(update).await?;
        manager.alter_table(al).await?;

        Ok(())
    }
}
