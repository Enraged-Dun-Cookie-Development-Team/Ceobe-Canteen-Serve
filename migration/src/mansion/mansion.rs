use sea_schema::migration::prelude::*;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "Mansion-migration"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
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
                            .default(0),
                    )
                    .col(ColumnDef::new(Mansion::CreateTime).date_time().not_null())
                    .col(ColumnDef::new(Mansion::EditTime).date_time().not_null())
                    .col(ColumnDef::new(Mansion::Link).string_len(128).not_null())
                    .index(
                        Index::create()
                            .col(Mansion::Mid)
                            .col(Mansion::SubMid)
                            .name("mansion_id"),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Mansion::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
pub enum Mansion {
    Table,
    Id,
    Mid,
    SubMid,
    CreateTime,
    EditTime,
    Link,
}
