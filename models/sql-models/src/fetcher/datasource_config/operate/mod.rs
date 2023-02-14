pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod verify;

use sea_orm::FromQueryResult;
use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::GetDatabaseConnect,
};
use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

use crate::fetcher::FetcherOperate;

pub struct Datasource<'c, C>(&'c C);

impl<'c, C> Datasource<'c, C> {
    fn get_connect(&self) -> &C::Connect
    where
        C: GetDatabaseConnect,
    {
        self.0.get_connect()
    }
}

impl<'c, C> SubOperate<'c> for Datasource<'c, C> {
    type Parent = FetcherOperate<'c, C>;

    fn from_parent(parent: &'c Self::Parent) -> Self { Self(parent.0) }
}

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("蹲饼器 Datasource {0} 不存在")]
    #[status_err(err(
        err_code = 0x0007,
        prefix = "ErrPrefix::NOT_FOUND",
        resp_msg = "蹲饼器数据源不存在"
    ))]
    DatasourceNotFound(i32),
    #[error("Datasources有不存在")]
    #[status_err(err(
        err_code = 0x0008,
        prefix = "ErrPrefix::NOT_FOUND",
        resp_msg = "有不存在的数据源，请刷新重新配置"
    ))]
    DatasourcesNotFound,
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(FromQueryResult)]
struct PlatformDatasource {
    pub(crate) platform: String,
}
