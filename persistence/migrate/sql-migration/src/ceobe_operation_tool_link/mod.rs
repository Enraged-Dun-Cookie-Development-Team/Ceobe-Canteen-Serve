use sea_orm_migration::prelude::*;

pub mod m20231018_162927_create;

#[derive(Debug, Iden)]
enum CeobeOperationToolLink {
    Table,
    Id,
    Nickname,
    Avatar,
    JumpUrl,
}
