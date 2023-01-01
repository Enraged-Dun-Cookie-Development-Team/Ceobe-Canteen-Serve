pub mod m20221217_155140_create;
use sea_orm_migration::prelude::*;

#[derive(Debug, Iden)]
enum FetcherConfig {
    Table,
    Id,
    LiveNumber,
    FetcherCount,
    GroupName,
    Platform,
    DatasourceId,
    Interval,
    IntervalByTimeRange,
}
pub mod m20221228_232347_alter_default;
pub mod m20221231_150609_alter_interval;
pub mod m20221231_205557_alter_group_name;
pub mod m20230101_013601_create_index;
