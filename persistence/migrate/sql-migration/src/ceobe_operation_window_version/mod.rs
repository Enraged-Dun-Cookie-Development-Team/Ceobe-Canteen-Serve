use sea_orm_migration::prelude::*;
pub mod m20230729_211229_create;

#[derive(Debug, Iden)]
enum CeobeOperationWindowVersion {
    Table,
    Id,
    Version,
    Force,
    LastForceVersion,
    Description,
    Exe,
    SpareExe,
    Dmg,
    SpareDmg,
    Baidu,
    BaiduText,
}