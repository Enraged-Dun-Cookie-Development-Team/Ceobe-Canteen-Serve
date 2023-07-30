use sea_orm_migration::prelude::*;

use super::CeobeOperationAppVersion;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230729_212539_ceobe_operation_app_version_add_download_link"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(CeobeOperationAppVersion::Table)
            .add_column(
                ColumnDef::new(CeobeOperationAppVersion::Apk)
                    .string_len(256)
                    .default("")
                    .not_null(),
            )
            .add_column(
                ColumnDef::new(CeobeOperationAppVersion::SpareApk)
                    .string_len(256)
                    .default("")
                    .not_null(),
            )
            .add_column(
                ColumnDef::new(CeobeOperationAppVersion::Baidu)
                    .string_len(256)
                    .default("")
                    .not_null(),
            )
            .add_column(
                ColumnDef::new(CeobeOperationAppVersion::BaiduText)
                    .string_len(32)
                    .default("")
                    .not_null(),
            );

        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(CeobeOperationAppVersion::Table)
            .drop_column(CeobeOperationAppVersion::Apk)
            .drop_column(CeobeOperationAppVersion::SpareApk)
            .drop_column(CeobeOperationAppVersion::Baidu)
            .drop_column(CeobeOperationAppVersion::BaiduText);
        manager.alter_table(al).await?;

        Ok(())
    }
}
