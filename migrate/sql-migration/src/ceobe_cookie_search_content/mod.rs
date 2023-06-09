use sea_orm_migration::prelude::*;
pub mod m20230606_135941_create;

#[derive(Debug, Iden)]
enum CeobeCookieSearchContent {
    Table,
    Id,
    ObjectId,
    SourceConfigId,
    Content,
}
