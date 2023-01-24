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
    fn get_connect(&'c self) -> &'c C::Connect<'c>
    where
        C: GetDatabaseConnect,
    {
        self.0.get_connect()
    }
}

impl<'op, C> SubOperate<'op> for Datasource<'op, C> {
    type Parent = FetcherOperate<'op, C>;

    fn from_parent(parent: &'op mut Self::Parent) -> Self {
        Self(parent.0)
    }
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
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(FromQueryResult)]
struct PlatformDatasource {
    pub(crate) platform: String,
}
