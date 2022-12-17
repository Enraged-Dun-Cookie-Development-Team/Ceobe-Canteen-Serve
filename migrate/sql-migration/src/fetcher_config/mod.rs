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