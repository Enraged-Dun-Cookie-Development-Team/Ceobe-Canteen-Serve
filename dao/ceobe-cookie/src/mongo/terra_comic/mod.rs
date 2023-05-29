pub mod retrieve;
use std::ops::Deref;

use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    mongo_connection::MongoDbError,
    StatusErr, ThisError,
};

use crate::CookieDatabaseOperate;

pub struct TerraComicOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for TerraComicOperate<'db, Conn> {
    type Parent = CookieDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for TerraComicOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> CookieDatabaseOperate<'db, Conn> {
    pub fn terra_comic(&self) -> TerraComicOperate<'_, Conn> { self.child() }
}
