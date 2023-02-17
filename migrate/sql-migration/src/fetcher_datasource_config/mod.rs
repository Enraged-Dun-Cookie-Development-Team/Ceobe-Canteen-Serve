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
    DbUniqueKey,
    DeleteAt
}
pub mod m20221231_200206_alter_nickname;
pub mod m20230217_135012_add_sort_detele;
