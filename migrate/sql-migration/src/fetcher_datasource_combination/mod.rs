use sea_orm_migration::prelude::*;

pub mod m20230310_213209_create;

#[derive(Debug, Iden)]
enum FetcherDatasourceCombination {
    Table,
    Id,
    CombinationId,
    Bitmap1,
    Bitmap2,
    Bitmap3,
    Bitmap4,
}