pub mod checked;
pub mod checkers;
pub mod mongo_db;
pub mod operate;

pub mod preludes {
    pub use super::{checked::*, checkers::*, mongo_db::*};
}
use mongo_connection::MongoDbError;
use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MansionDataError {
    #[error("Mongo数据库异常")]
    Db(#[from] MongoDbError),

    #[error("指定饼学大厦ID未找到")]
    MansionNotFound,
    #[error("指定ID:[{mansion_id:?}] 的饼学大厦已经存在")]
    MansionIdExist { mansion_id: String },
}

impl status_err::StatusErr for MansionDataError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            MansionDataError::Db(db) => db.prefix(),

            MansionDataError::MansionNotFound => ErrPrefix::NOT_FOUND,
            MansionDataError::MansionIdExist { mansion_id: _ } => {
                ErrPrefix::CHECKER
            }
        }
    }

    fn code(&self) -> u16 {
        match self {
            MansionDataError::Db(db) => db.code(),

            MansionDataError::MansionNotFound => 0x0001,
            MansionDataError::MansionIdExist { mansion_id: _ } => 0x0008,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            MansionDataError::Db(db) => db.http_code(),

            MansionDataError::MansionNotFound => HttpCode::NOT_FOUND,
            MansionDataError::MansionIdExist { mansion_id: _ } => {
                HttpCode::CONFLICT
            }
        }
    }
}
