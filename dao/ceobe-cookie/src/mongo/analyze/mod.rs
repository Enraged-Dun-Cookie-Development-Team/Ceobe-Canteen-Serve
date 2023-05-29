pub mod retrieve;
use std::ops::Deref;

use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    mongo_connection::MongoDbError,
    mongodb::bson,
    StatusErr, ThisError,
};

use crate::CookieDatabaseOperate;

pub struct AnalyzeOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for AnalyzeOperate<'db, Conn> {
    type Parent = CookieDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for AnalyzeOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),

    #[error("bson序列化错误")]
    MongoDeError(#[from] bson::de::Error),
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> CookieDatabaseOperate<'db, Conn> {
    pub fn analyze(&self) -> AnalyzeOperate<'_, Conn> { self.child() }
}
