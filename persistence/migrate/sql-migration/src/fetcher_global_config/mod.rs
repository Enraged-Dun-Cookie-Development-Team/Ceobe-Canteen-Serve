use sea_orm_migration::prelude::*;

pub mod m20221217_155049_create;

#[derive(Debug, Iden)]
enum FetcherGlobalConfig {
    Table,
    Id,
    Key,
    Value,
}
