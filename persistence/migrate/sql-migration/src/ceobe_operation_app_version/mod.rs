pub mod m20220803_104932_create;
pub mod m20221218_001756_charset_and_collate;
pub mod m20230729_212539_add_download_link;

use sea_orm_migration::prelude::*;

#[derive(Debug, Iden)]
enum CeobeOperationAppVersion {
    Table,
    Apk,
    SpareApk,
    Baidu,
    BaiduText,
}
