use sea_schema::migration::prelude::*;

use super::mansion::Mansion;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "20220306165019-EachMansion-migration"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(EachMansion::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(EachMansion::Id)
                    .big_integer()
                    .primary_key()
                    .unique_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(EachMansion::Mid).big_integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(EachMansion::Table, EachMansion::Mid)
                    .to(Mansion::Table, Mansion::Id),
            )
            .col(
                ColumnDef::new(EachMansion::Date)
                    .date()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(EachMansion::Content)
                    .text()
                    .default(Value::Date(None)),
            );
        manager.create_table(table).await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(EachMansion::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
#[derive(Iden)]
pub enum EachMansion {
    Table,
    Id,
    Mid,
    Date,
    Content,
}
