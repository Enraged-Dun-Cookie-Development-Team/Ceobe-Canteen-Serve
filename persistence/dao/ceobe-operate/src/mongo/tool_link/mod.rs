use std::ops::Deref;

pub use db_ops_prelude::mongo_models::ceobe::operation::tool_link::*;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    mongo_connection::MongoDbError,
    StatusErr, ThisError,
};

use crate::OperationDatabaseOperate;

mod create;
mod delete;
mod retrieve;
mod update;

pub struct ToolLinkOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for ToolLinkOperate<'db, Conn> {
    type Parent = OperationDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for ToolLinkOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateMongoError {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),
    #[error("mongo 查询异常{0}")]
    Find(#[from] db_ops_prelude::mongodb::error::Error),
}

type OperateResult<T> = Result<T, OperateMongoError>;

impl<'db, Conn> OperationDatabaseOperate<'db, Conn> {
    pub fn tool_link_mongo(&self) -> ToolLinkOperate<'_, Conn> {
        self.child()
    }
}
