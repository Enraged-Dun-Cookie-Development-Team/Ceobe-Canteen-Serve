pub mod create;
pub mod delete;
pub mod retrieve;
pub mod verify;

use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::GetDatabaseConnect,
};
use status_err::StatusErr;
use thiserror::Error;

use crate::fetcher::FetcherOperate;

pub struct Config<'c, C: 'c>(&'c C);

impl<'c, C: 'c> Config<'c, C> {
    pub(self) fn get_connect(&'c self) -> &'c C::Connect<'c>
    where
        C: GetDatabaseConnect,
    {
        self.0.get_connect()
    }
}

impl<'c, C: 'c> SubOperate<'c> for Config<'c, C> {
    type Parent = FetcherOperate<'c, C>;

    fn from_parent(parent: &'c mut Self::Parent) -> Self {
        Self(parent.0)
    }
}

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;
