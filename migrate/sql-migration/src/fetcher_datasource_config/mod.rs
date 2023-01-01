pub mod m20221217_155038_create;
use sea_orm_migration::prelude::*;

#[derive(Debug, Iden)]
enum FetcherDatasourceConfig {
    Table,
    Id,
    Platform,
    Datasource,
    Nickname,
    Avatar,
    Config,
    UniqueId,
}
pub mod m20221231_200206_alter_nickname;
