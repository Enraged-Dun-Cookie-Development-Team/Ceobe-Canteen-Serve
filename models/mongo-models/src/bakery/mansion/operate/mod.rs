use mongo_connection::{MongoDbError};



mod create;
mod delete;
mod retrieve;
mod update;
mod verify;
use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;
pub struct MansionDataMongoOperate;
pub use OperateError::*;
pub type MongoErr = mongodb::error::Error;


#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("Mongo数据库异常")]
    Db(#[from] MongoDbError),

    #[error("指定饼学大厦ID未找到")]
    #[status_err(err(prefix = "ErrPrefix::NOT_FOUND", err_code = 0x0001,))]
    MansionNotFound,
    #[error("指定ID:[{0:?}] 的饼学大厦已经存在")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0008,
        http_code = "HttpCode::CONFLICT"
    ))]
    MansionIdExist(String),
}
