use sea_orm_migration::prelude::*;

pub mod m20221217_155027_create;

#[derive(Debug, Iden)]
enum FetcherPlatformConfig {
    Table,
    Id,
    TypeId,
    PlatformName,
    MinRequestInterval,
}
pub mod m20221231_150609_alter_min_interval;
