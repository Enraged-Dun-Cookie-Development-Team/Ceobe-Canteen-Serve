pub mod m20221217_155049_create;
use sea_orm_migration::prelude::*;


#[derive(Debug, Iden)]
enum FetcherGlobalConfig {
    Table,
    Id,
    Key,
    Value,
}
