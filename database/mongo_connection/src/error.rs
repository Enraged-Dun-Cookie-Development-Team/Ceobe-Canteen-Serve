use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

use crate::MongoErr;

#[derive(Debug, Error)]
pub enum MongoDbError {
    #[error("MongoDb 数据库异常 {0}")]
    Mongo(#[from] MongoErr),
    #[error("指定集合不存在{0:?}")]
    CollectionNotFound(&'static str),
}

impl StatusErr for MongoDbError {
    fn prefix(&self) -> ErrPrefix { ErrPrefix::MONGO_DB }

    fn code(&self) -> u16 {
        match self {
            MongoDbError::Mongo(err) => err.code(),
            MongoDbError::CollectionNotFound(_) => 0x0013,
        }
    }
}
