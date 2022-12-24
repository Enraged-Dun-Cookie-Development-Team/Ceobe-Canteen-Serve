pub mod create;
pub mod update;
pub mod retrieve;
pub mod delete;
pub mod verify;

use sea_orm::FromQueryResult;
use thiserror::Error;
use status_err::{ErrPrefix, StatusErr};

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