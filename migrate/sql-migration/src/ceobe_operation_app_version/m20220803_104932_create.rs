
use sea_orm_migration::prelude::*;


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220803_104932_ceobe_operation_app_version_create" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(DbTable)
            .if_not_exists()
            .col(
                ColumnDef::new(Id)
                    .integer()
                    .auto_increment()
                    .primary_key()
                    .not_null(),
            )
            .col(ColumnDef::new(Version).char_len(11).not_null())
            .col(ColumnDef::new(Force).boolean().not_null().default(false))
            .col(ColumnDef::new(LastForceVersion).char_len(11).not_null())
            .col(ColumnDef::new(Description).text().not_null().default(""))
            .col(
                ColumnDef::new(CreateAt)
                    .date_time()
                    .not_null()
                    .default(get_now_naive_date_time()),
            )
            .col(
                ColumnDef::new(ModifyAt)
                    .date_time()
                    .not_null()
                    .default(get_now_naive_date_time()),
            )
            .col(
                ColumnDef::new(DeleteAt)
                    .date_time()
                    .not_null()
                    .default(get_zero_data_time()),
            );

        // 添加唯一索引，方便查询，更新软删除条目则取消软删除
        table.index(
            Index::create()
                .col(Version)
                .name("version")
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
pub(super) use CeobeOperationAppVersion::{Table as DbTable, *};
use sql_models::{get_zero_data_time, get_now_naive_date_time};

#[derive(Debug, Iden)]
pub(super) enum CeobeOperationAppVersion {
    Table,
    Id,
    Version,
    Force,
    LastForceVersion,
    Description,
    // soft delete
    CreateAt,
    ModifyAt,
    DeleteAt,
}

