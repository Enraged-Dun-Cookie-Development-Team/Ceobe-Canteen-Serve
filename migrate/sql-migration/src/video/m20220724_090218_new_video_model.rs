use chrono::NaiveDateTime;
use sea_orm_migration::{prelude::*, sea_query::Table};

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220724_090218_new_video_model" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .table(CeobeOperationVideo::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CeobeOperationVideo::Id)
                    .integer()
                    .primary_key()
                    .auto_increment(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::Bv)
                    .char_len(12)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::StartTime)
                    .date_time()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::OverTime)
                    .date_time()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::Title)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::Author)
                    .string_len(128)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::VideoLink)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::CoverImage)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CeobeOperationVideo::DeleteAt)
                    .date_time()
                    .not_null()
                    .default(Value::ChronoDateTime(
                        Box::new(NaiveDateTime::from_timestamp(0, 0)).into(),
                    )),
            );
        // 添加唯一索引，用于软删除
        table.index(
            Index::create()
                .col(CeobeOperationVideo::Bv)
                .col(CeobeOperationVideo::DeleteAt)
                .name("mark-delete-id")
                .unique(),
        );

        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::drop();
        table.table(CeobeOperationVideo::Table);
        manager.drop_table(table).await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum CeobeOperationVideo {
    Table,
    Id,
    // BV1Rg411Z7LV
    // BV1ZB4y1Y7Hm
    Bv,
    StartTime,
    OverTime,
    Title,
    Author,
    VideoLink,
    CoverImage,
    // soft delete
    DeleteAt,
}
