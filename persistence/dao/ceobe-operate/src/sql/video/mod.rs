use std::ops::Deref;

pub use db_ops_prelude::sql_models::ceobe_operation::video::*;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm::DbErr,
    StatusErr, ThisError,
};

use crate::OperationDatabaseOperate;

mod retrieve;
mod update;

pub struct VideoOperate<'s, Conn>(&'s Conn);

impl<'s, Conn> SubOperate<'s> for VideoOperate<'s, Conn> {
    type Parent = OperationDatabaseOperate<'s, Conn>;

    fn from_parent(parent: &'s Self::Parent) -> Self { Self(parent) }
}

impl<'s, Conn> Deref for VideoOperate<'s, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] DbErr),
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> OperationDatabaseOperate<'db, Conn> {
    pub fn video(&self) -> VideoOperate<'_, Conn> { self.child() }
}
