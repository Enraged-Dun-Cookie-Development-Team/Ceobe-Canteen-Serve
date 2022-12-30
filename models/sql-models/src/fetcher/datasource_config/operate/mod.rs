pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod verify;

use sea_orm::FromQueryResult;
use status_err::StatusErr;
use thiserror::Error;

pub struct FetcherDatasourceConfigSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(FromQueryResult)]
struct DatasourceCounts {
    pub(crate) count: i64,
}

#[derive(FromQueryResult)]
struct PlatformDatasource {
    pub(crate) platform: String,
}
