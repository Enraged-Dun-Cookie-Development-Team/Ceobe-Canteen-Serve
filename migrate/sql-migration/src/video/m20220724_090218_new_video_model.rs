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
            .table(VideoData::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(VideoData::Id)
                    .integer()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(VideoData::StartTime).date_time().not_null())
            .col(ColumnDef::new(VideoData::OverTime).date_time().not_null())
            .col(ColumnDef::new(VideoData::Title).string_len(256).not_null())
            .col(ColumnDef::new(VideoData::Author).string_len(128).not_null())
            .col(
                ColumnDef::new(VideoData::VideoLink)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(VideoData::CoverImage)
                    .string_len(256)
                    .not_null(),
            )
            .col(
                ColumnDef::new(VideoData::DeleteAt)
                    .date_time()
                    .not_null()
                    .default(Value::ChronoDateTime(
                        Box::new(NaiveDateTime::from_timestamp(0, 0)).into(),
                    )),
            );
        // 添加唯一索引，用于软删除
        table.index(
            Index::create()
                .col(VideoData::Title)
                .col(VideoData::DeleteAt)
                .name("mark-delete-id")
                .unique(),
        );

        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::drop();
        table.table(VideoData::Table);
        manager.drop_table(table).await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum VideoData {
    Table,
    Id,
    StartTime,
    OverTime,
    Title,
    Author,
    VideoLink,
    CoverImage,
    // soft delete
    DeleteAt,
}
