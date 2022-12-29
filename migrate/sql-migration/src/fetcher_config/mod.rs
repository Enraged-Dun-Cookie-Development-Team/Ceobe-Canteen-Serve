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
