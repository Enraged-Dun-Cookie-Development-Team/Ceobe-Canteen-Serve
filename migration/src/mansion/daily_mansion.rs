use sea_schema::migration::prelude::*;

use super::mansion::Mansion;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "20220306165019-DailyMansion-migration"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(DailyMansion::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(DailyMansion::Id)
                    .big_integer()
                    .primary_key()
                    .unique_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(DailyMansion::Mid).big_integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from_col(DailyMansion::Mid)
                    .to(Mansion::Table, Mansion::Id),
            )
            .col(
                ColumnDef::new(DailyMansion::Date)
                    .date()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(DailyMansion::Content)
                    .text()
                    .default(Value::Date(None)),
            );
        manager.create_table(table).await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(DailyMansion::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
#[derive(Iden)]
pub enum DailyMansion {
    Table,
    Id,
    Mid,
    Date,
    Content,
}
