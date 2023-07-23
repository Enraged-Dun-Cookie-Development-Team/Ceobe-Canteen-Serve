use std::ops::Deref;

pub use db_ops_prelude::sql_models::ceobe_operation::announcement::*;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm, StatusErr, ThisError,
};

use crate::OperationDatabaseOperate;

mod delete;
mod retrieve;
mod update;

pub struct AnnouncementOperate<'c, C>(&'c C);

impl<'c, C> Deref for AnnouncementOperate<'c, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'c, C> SubOperate<'c> for AnnouncementOperate<'c, C> {
    type Parent = OperationDatabaseOperate<'c, C>;

    fn from_parent(parent: &'c Self::Parent) -> Self { Self(parent) }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> OperationDatabaseOperate<'db, Conn> {
    pub fn announcement(&self) -> AnnouncementOperate<'_, Conn> {
        self.child()
    }
}
