pub mod check;
pub mod mongo_db;
pub mod operate;
pub mod checkers;

pub mod preludes {
    pub use super::{check::*, mongo_db::*};
}
use mongo_connection::MongoDbError;
use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MansionDataError {
    #[error("Mongo数据库异常")]
    Db(#[from] MongoDbError),
    #[error("饼学大厦id格式不正确")]
    UnknownMansionId,
    #[error("指定饼学大厦ID未找到")]
    MansionNotFound,
    #[error("指定ID:[{mansion_id:?}] 的饼学大厦已经存在")]
    MansionIdExist { mansion_id: String },
    #[error("未知的预期确信度等级")]
    UnknownPredictType,
    #[error("错误的Fraction值范围(0~5)")]
    BadFraction
}

impl status_err::StatusErr for MansionDataError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            MansionDataError::Db(db) => db.prefix(),
            
            MansionDataError::UnknownMansionId => ErrPrefix::CHECKER,
            MansionDataError::MansionNotFound => ErrPrefix::NOT_FOUND,
            MansionDataError::MansionIdExist { mansion_id: _ } => {
                ErrPrefix::CHECKER
            },
            MansionDataError::UnknownPredictType => ErrPrefix::CHECKER,
            MansionDataError::BadFraction => ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        match self {
            MansionDataError::Db(db) => db.code(),
            
            MansionDataError::UnknownMansionId => 0x0002,
            MansionDataError::MansionNotFound => 0x0001,
            MansionDataError::MansionIdExist { mansion_id: _ } => 0x0008,
            MansionDataError::UnknownPredictType => 0x0006,
            MansionDataError::BadFraction => 0x0003,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            MansionDataError::Db(db) => db.http_code(),
            
            MansionDataError::UnknownMansionId => HttpCode::NOT_ACCEPTABLE,
            MansionDataError::MansionNotFound => HttpCode::NOT_FOUND,
            MansionDataError::MansionIdExist { mansion_id: _ } => {
                HttpCode::CONFLICT
            },
            MansionDataError::UnknownPredictType => HttpCode::NOT_ACCEPTABLE,
            MansionDataError::BadFraction => HttpCode::NOT_FOUND,
        }
    }
}
