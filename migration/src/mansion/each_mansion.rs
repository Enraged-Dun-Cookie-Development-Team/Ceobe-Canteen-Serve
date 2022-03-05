use sea_schema::migration::prelude::*;

use super::mansion::Mansion;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "EachMansion-migration"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
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
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(EachMansion::Table, EachMansion::Mid)
                            .to(Mansion::Table, Mansion::Id),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(EachMansion::Table)
                    .to_owned(),
            )
            .await
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
