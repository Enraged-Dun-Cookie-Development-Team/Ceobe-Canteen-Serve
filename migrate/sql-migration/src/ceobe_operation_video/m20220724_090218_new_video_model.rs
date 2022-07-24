use sea_orm_migration::prelude::*;
pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220724_090218_new_video_model" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(DbTable)
            .if_not_exists()
            .col(ColumnDef::new(Id).integer().primary_key().auto_increment())
            .col(ColumnDef::new(Bv).char_len(12).not_null().unique_key())
            .col(ColumnDef::new(Order).integer().not_null())
            .col(ColumnDef::new(StartTime).date_time().not_null())
            .col(ColumnDef::new(OverTime).date_time().not_null())
            .col(ColumnDef::new(Title).string_len(256).not_null())
            .col(ColumnDef::new(Author).string_len(128).not_null())
            .col(ColumnDef::new(VideoLink).string_len(256).not_null())
            .col(ColumnDef::new(CoverImage).string_len(256).not_null())
            .col(
                ColumnDef::new(DeleteAt)
                    .date_time()
                    .not_null()
                    .default(get_zero_data_time()),
            );
        // 添加唯一索引，用于软删除
        table.index(
            Index::create()
                .col(Order)
                .col(DeleteAt)
                .name("mark-delete-id")
                .unique(),
        );

        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(DbTable);
        manager.drop_table(table).await?;
        Ok(())
    }
}
use sql_models::get_zero_data_time;
pub use CeobeOperationVideo::{Table as DbTable, *};

#[derive(Iden)]
pub enum CeobeOperationVideo {
    Table,
    Id,
    Bv,
    Order,
    StartTime,
    OverTime,
    Title,
    Author,
    VideoLink,
    CoverImage,
    // soft delete
    DeleteAt,
}
