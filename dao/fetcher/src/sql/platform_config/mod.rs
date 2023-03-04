pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod verify;

use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::GetDatabaseConnect,
};
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

use crate::fetcher::FetcherOperate;

pub struct Platform<'c, C>(&'c C);

impl<C: GetDatabaseConnect> GetDatabaseConnect for Platform<'_, C> {
    type Connect = C::Connect;

    fn get_connect(&self) -> &Self::Connect { self.0.get_connect() }
}

impl<'c, C> SubOperate<'c> for Platform<'c, C> {
    type Parent = FetcherOperate<'c, C>;

    fn from_parent(parent: &'c Self::Parent) -> Self { Self(parent.0) }
}

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("该平台下存在数据源，无法删除平台")]
    #[status_err(err(
        err_code = 0x00_12,
        prefix = "ErrPrefix::CHECKER",
        http_code = "HttpCode::CONFLICT"
    ))]
    NoDeletePlatformHasDatasource,
}

#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;
