use sea_schema::migration::prelude::*;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "20220306163810-Mansion-migration"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(Mansion::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Mansion::Id)
                    .big_integer()
                    .primary_key()
                    .unique_key()
                    .not_null()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Mansion::Mid).integer().not_null())
            .col(
                ColumnDef::new(Mansion::SubMid)
                    .integer()
                    .not_null()
                    .default(0i32),
            )
            .col(ColumnDef::new(Mansion::StartAt).date().not_null())
            .col(ColumnDef::new(Mansion::EndAt).date().not_null())
            .col(ColumnDef::new(Mansion::Link).string_len(128).not_null())
            .index(
                Index::create()
                    .col(Mansion::Mid)
                    .col(Mansion::SubMid)
                    .name("mansion_id"),
            );
        manager.create_table(table).await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table=sea_query::Table::drop();
        table.table(Mansion::Table);
        manager.drop_table(table).await?;
        

        Ok(())
    }
}
#[derive(Iden)]
pub enum Mansion {
    Table,
    Id,
    Mid,
    SubMid,
    StartAt,
    EndAt,
    Link,
}
