use std::ops::Deref;

pub use db_ops_prelude::sql_models::ceobe_operation::desktop_version::*;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    get_connect::GetDatabaseConnect,
    sea_orm, ErrPrefix, HttpCode, StatusErr, ThisError,
};

use crate::OperationDatabaseOperate;

mod create;
mod retrieve;
mod verify;

pub struct DesktopVersionOperate<'c, C: 'c + GetDatabaseConnect>(
    &'c C::Connect,
);

impl<'c, C> Deref for DesktopVersionOperate<'c, C>
where
    C: 'c + GetDatabaseConnect,
{
    type Target = C::Connect;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'c, C> SubOperate<'c> for DesktopVersionOperate<'c, C>
where
    C: GetDatabaseConnect,
{
    type Parent = OperationDatabaseOperate<'c, C>;

    fn from_parent(parent: &'c Self::Parent) -> Self {
        Self(parent.get_connect())
    }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("Desktop指定版本:[{0:?}]信息已经存在")]
    #[status_err(err(
        err_code = 0x000B,
        prefix = "ErrPrefix::CHECKER",
        http_code = "HttpCode::CONFLICT"
    ))]
    DesktopVersionIdExist(String),
    #[error("Desktop指定版本:[{0:?}]信息不存在")]
    #[status_err(err(err_code = 0x0004, prefix = "ErrPrefix::NOT_FOUND",))]
    DesktopVersionIdNoExist(String),
    #[error("还没有Desktop版本信息")]
    #[status_err(err(err_code = 0x0005, prefix = "ErrPrefix::NOT_FOUND",))]
    NotDesktopVersion,
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn: GetDatabaseConnect> OperationDatabaseOperate<'db, Conn> {
    pub fn desktop_version(&self) -> DesktopVersionOperate<'_, Conn> {
        self.child()
    }
}
