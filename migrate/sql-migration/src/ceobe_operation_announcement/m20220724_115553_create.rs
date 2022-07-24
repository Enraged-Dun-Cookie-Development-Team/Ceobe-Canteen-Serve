use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220724_115553_create"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(CeobeOperationAnnouncement::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::StartTime)
                    .date_time()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::OverTime)
                    .date_time()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::ImgUrl)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::Content)
                    .text()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::Order)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::Notice)
                    .boolean()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationAnnouncement::DeleteAt)
                    .date_time()
                    .not_null()
                    .default(Value::ChronoDateTime(
                        Box::new(NaiveDateTime::from_timestamp(0, 0)).into(),
                    )),
            );
        table.index(
            Index::create()
                .col(CeobeOperationAnnouncement::Order)
                .col(CeobeOperationAnnouncement::DeleteAt)
                .name("mark-delete-id")
                .unique(),
        );
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(CeobeOperationAnnouncement::Table);
        manager.drop_table(table).await?;

        Ok(())
    }
}

#[derive(Debug, Iden)]
pub(super) enum CeobeOperationAnnouncement {
    Table,
    Id,
    StartTime,
    OverTime,
    ImgUrl,
    Content,
    Order,
    Notice,
    // soft delete
    DeleteAt,
}
