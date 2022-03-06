use sea_schema::migration::prelude::*;

use super::each_mansion::EachMansion;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "20220306165258-InnerMansion-migration"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(InnerMansion::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(InnerMansion::Id)
                    .primary_key()
                    .big_integer()
                    .not_null()
                    .unique_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(InnerMansion::Eid).big_integer().not_null())
            .col(
                ColumnDef::new(InnerMansion::PredictLevel)
                    .enumeration("predict", ["false", "unknown", "true"])
                    .not_null(),
            )
            .col(ColumnDef::new(InnerMansion::Info).text().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(InnerMansion::Table, InnerMansion::Eid)
                    .to(EachMansion::Table, EachMansion::Id),
            );
        manager.create_table(table).await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(InnerMansion::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
#[derive(Iden)]
pub enum InnerMansion {
    Table,
    Id,
    Eid,
    PredictLevel,
    Info,
}
