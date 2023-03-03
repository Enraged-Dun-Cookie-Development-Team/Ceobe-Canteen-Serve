mod create;
mod delete;
mod retrieve;
mod update;

use std::ops::Deref;

pub use db_ops_prelude::sql_models::ceobe_operation::resource::*;
use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    sea_orm, StatusErr, ThisError,
};

pub struct ResourceOperate<'op, C: 'op>(&'op C);

impl<'op, C: 'op> Deref for ResourceOperate<'op, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'c, C> SubOperate<'c> for ResourceOperate<'c, C> {
    type Parent = OperationDatabaseOperate<'c, C>;

    fn from_parent(parent: &Self::Parent) -> Self { Self(parent.0) }
}

use status_err::{ErrPrefix, HttpCode};

use crate::OperationDatabaseOperate;

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("存在多个可用的资源全可用的记录")]
    #[status_err(err(
        prefix = r#"ErrPrefix::CHECKER"#,
        err_code = 0x00_0D,
        http_code = "HttpCode::INTERNAL_SERVER_ERROR"
    ))]
    MultiAllAvailable,
    #[error("没有可用的资源全可用的记录")]
    #[status_err(err(
        prefix = r#"ErrPrefix::NOT_FOUND"#,
        err_code = 0x00_06,
    ))]
    NoneAllAvailable,
}
type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> OperationDatabaseOperate<'db, Conn> {
    pub fn resource(&self) -> ResourceOperate<'_, Conn> { self.child() }
}
