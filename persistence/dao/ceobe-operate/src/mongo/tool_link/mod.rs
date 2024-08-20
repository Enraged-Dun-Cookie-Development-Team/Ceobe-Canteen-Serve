use std::ops::Deref;

pub use db_ops_prelude::mongo_models::ceobe::operation::tool_link::*;
use db_ops_prelude::{database_operates::sub_operate::{SubOperate, SuperOperate}, mongo_connection::MongoDbError, mongodb, StatusErr, ThisError};
use db_ops_prelude::mongodb::error::Error;
use crate::OperationDatabaseOperate;

mod create;
mod delete;
mod retrieve;
mod update;

pub use crate::common::tool_link::ToolLinkOperate;


#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),
}

impl From<mongodb::error::Error> for OperateError {
    fn from(value: Error) -> Self {
        MongoDbError::from(value).into()
    }
}

type OperateResult<T> = Result<T, OperateError>;

