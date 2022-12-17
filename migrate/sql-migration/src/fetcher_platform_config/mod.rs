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
