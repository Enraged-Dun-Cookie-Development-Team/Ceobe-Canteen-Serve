use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

use crate::MongoErr;

#[derive(Debug, Error, StatusErr)]
pub enum MongoDbError {
    #[error("MongoDb 数据库异常 {0}")]
    Mongo(#[from] MongoErr),
    #[error("指定集合不存在{0:?}")]
    #[status_err(err(err_code = 0x0013, prefix = "ErrPrefix::MONGO_DB"))]
    CollectionNotFound(&'static str),
}
