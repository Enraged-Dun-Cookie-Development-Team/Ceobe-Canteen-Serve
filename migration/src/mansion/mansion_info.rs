use sea_schema::migration::prelude::*;

use super::daily_mansion::DailyMansion;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "20220306165258-MansionInfo-migration"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(MansionInfo::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(MansionInfo::Id)
                    .primary_key()
                    .big_integer()
                    .not_null()
                    .unique_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(MansionInfo::Eid).big_integer().not_null())
            .col(
                ColumnDef::new(MansionInfo::PredictLevel)
                    .enumeration("predict", ["false", "unknown", "true"])
                    .not_null(),
            )
            .col(ColumnDef::new(MansionInfo::Info).text().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from_col(MansionInfo::Eid)
                    .to_col( DailyMansion::Id),
            );
        manager.create_table(table).await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(MansionInfo::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}
#[derive(Iden)]
pub enum MansionInfo {
    Table,
    Id,
    Eid,
    PredictLevel,
    Info,
}
