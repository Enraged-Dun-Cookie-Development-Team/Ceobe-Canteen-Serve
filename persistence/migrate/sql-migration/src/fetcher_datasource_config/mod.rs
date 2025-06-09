use sea_orm_migration::prelude::*;

pub mod m20221217_155038_create;

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
    DeleteAt,
    JumpUrl,
    Visual,
}
pub mod m20221231_200206_alter_nickname;
pub mod m20230217_135012_add_sort_detele;
pub mod m20230528_110010_add_jump_url;
pub mod m20231115_135841_add_visual;
