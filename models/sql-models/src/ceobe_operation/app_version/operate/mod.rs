pub mod create;
pub mod retrieve;
pub mod verify;

use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::GetDatabaseConnect,
};
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

pub struct AppVersionOperate<'c, C: 'c + GetDatabaseConnect>(
    &'c C::Connect,
);

impl<'c, C: 'c + GetDatabaseConnect> AppVersionOperate<'c, C> {
    pub(self) fn get_connect(&'c self) -> &C::Connect{
        self.0
    }
}

impl<'p: 'c, 'c, C: 'p> SubOperate<'p, 'c> for AppVersionOperate<'c, C>
where
    C: 'static + GetDatabaseConnect,
{
    type Parent<'parent> = SqlCeobeOperation<'parent, C>where 'parent:'c;

    fn from_parent<'parent: 'c>(parent: &'p Self::Parent<'parent>) -> Self {
        Self(parent.0.get_connect())
    }
}

use crate::ceobe_operation::SqlCeobeOperation;

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("App指定版本:[{0:?}]信息已经存在")]
    #[status_err(err(
        err_code = 0x000B,
        prefix = "ErrPrefix::CHECKER",
        http_code = "HttpCode::CONFLICT"
    ))]
    AppVersionIdExist(String),
    #[error("App指定版本:[{0:?}]信息不存在")]
    #[status_err(err(err_code = 0x0004, prefix = "ErrPrefix::NOT_FOUND",))]
    AppVersionIdNoExist(String),
    #[error("还没有App版本信息")]
    #[status_err(err(err_code = 0x0005, prefix = "ErrPrefix::NOT_FOUND",))]
    NotAppVersion,
}

type OperateResult<T> = Result<T, OperateError>;
