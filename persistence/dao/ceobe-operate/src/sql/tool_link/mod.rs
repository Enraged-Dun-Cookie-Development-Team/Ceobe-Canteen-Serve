use std::ops::Deref;

use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm, ErrPrefix, StatusErr, ThisError,
};

use crate::OperationDatabaseOperate;

mod create;
mod delete;
mod retrieve;
mod update;
mod verify;

pub struct ToolLinkOperate<'c, C>(&'c C);

impl<'c, C> Deref for ToolLinkOperate<'c, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'c, C> SubOperate<'c> for ToolLinkOperate<'c, C> {
    type Parent = OperationDatabaseOperate<'c, C>;

    fn from_parent(parent: &'c Self::Parent) -> Self { Self(parent) }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("工具链接 {0} 不存在")]
    #[status_err(err(
        err_code = 0x000A,
        prefix = "ErrPrefix::NOT_FOUND",
        resp_msg = "蹲饼器数据源不存在"
    ))]
    ToolLinkNotFound(i32),
}
type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> OperationDatabaseOperate<'db, Conn> {
    pub fn tool_link(&self) -> ToolLinkOperate<'_, Conn> { self.child() }
}
