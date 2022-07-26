use mongo_connection::{get_mongo_collection, CollectionGuard, MongoDbError};

use super::preludes::ModelMansion;

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

fn get_mansion_collection(
) -> Result<CollectionGuard<ModelMansion>, MongoDbError> {
    get_mongo_collection()
}

#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(Debug, Error)]
pub enum OperateError {
    #[error("Mongo数据库异常")]
    Db(#[from] MongoDbError),

    #[error("指定饼学大厦ID未找到")]
    MansionNotFound,
    #[error("指定ID:[{0:?}] 的饼学大厦已经存在")]
    MansionIdExist(String),
}

impl status_err::StatusErr for OperateError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            Db(db) => db.prefix(),
            MansionNotFound => ErrPrefix::NOT_FOUND,
            MansionIdExist(_) => ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        match self {
            Db(db) => db.code(),
            MansionNotFound => 0x0001,
            MansionIdExist(_) => 0x0008,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            Db(db) => db.http_code(),
            MansionNotFound => HttpCode::NOT_FOUND,
            MansionIdExist(_) => HttpCode::CONFLICT,
        }
    }
}
