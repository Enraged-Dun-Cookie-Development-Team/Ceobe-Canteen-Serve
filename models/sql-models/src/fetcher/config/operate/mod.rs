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

impl<C: GetDatabaseConnect> Config<'_, C> {
    fn get_connect(&self) -> &C::Connect {
        self.0.get_connect()
    }
}

impl<'p: 'c, 'c, C: 'static> SubOperate<'p, 'c> for Config<'c, C> {
    type Parent<'parent> = FetcherOperate<'parent, C>where 'parent:'c;

    fn from_parent<'parent: 'c>(parent: &'p Self::Parent<'parent>) -> Self {
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
