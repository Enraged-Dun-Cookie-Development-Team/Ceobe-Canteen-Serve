pub mod retrieve;
pub mod update;

use sql_connection::database_traits::{database_operates::sub_operate::SubOperate, get_connect::GetDatabaseConnect};
use status_err::StatusErr;
use thiserror::Error;

use crate::fetcher::FetcherOperate;

pub struct Global<'c,C>(&'c C);

impl<'c, C> GetDatabaseConnect for Global<'c, C> 
where C:GetDatabaseConnect
{
    type Connect<'s> = C::Connect<'s>
    where
        Self: 's;

    fn get_connect(&self) -> &Self::Connect<'_> {
        self.0.get_connect()
    }
}

impl<'c, C> SubOperate<'c> for Global<'c, C> {
    type Parent = FetcherOperate<'c,C>;

    fn from_parent(parent: &'c Self::Parent) -> Self {
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
