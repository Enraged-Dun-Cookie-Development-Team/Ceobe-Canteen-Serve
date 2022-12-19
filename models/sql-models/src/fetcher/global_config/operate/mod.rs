pub mod update;
pub mod retrieve;

use thiserror::Error;

pub struct FetcherGlobalConfigSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}

#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;