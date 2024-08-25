pub use db_ops_prelude::mongo_models::ceobe::operation::tool_link::*;
use db_ops_prelude::{
    mongo_connection::MongoDbError, mongodb, mongodb::error::Error,
    StatusErr, ThisError,
};

pub use crate::common::tool_link::ToolLinkOperate;

mod create;
mod delete;
mod retrieve;
mod update;

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),
    #[error(transparent)]
    Bson(#[from] mongodb::bson::ser::Error),
}

impl From<mongodb::error::Error> for OperateError {
    fn from(value: Error) -> Self { MongoDbError::from(value).into() }
}

type OperateResult<T> = Result<T, OperateError>;
