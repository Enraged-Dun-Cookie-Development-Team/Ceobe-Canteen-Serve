use std::ops::Deref;

use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm, StatusErr, ThisError,
};

use crate::CookieDatabaseOperate;

pub mod retrieve;

pub struct SearchContentOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for SearchContentOperate<'db, Conn> {
    type Parent = CookieDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for SearchContentOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> CookieDatabaseOperate<'db, Conn> {
    pub fn search_content(&self) -> SearchContentOperate<'_, Conn> {
        self.child()
    }
}
