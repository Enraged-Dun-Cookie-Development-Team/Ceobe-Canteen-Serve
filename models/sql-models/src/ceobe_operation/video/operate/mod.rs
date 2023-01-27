mod retrieve;
mod update;
use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::{GetDatabaseConnect, GetDatabaseTransaction},
};
use thiserror::Error;

use crate::ceobe_operation::SqlCeobeOperation;

pub struct VideoOperate<'c, C: 'c>(&'c C);

impl<'c, C: 'c> VideoOperate<'c, C> {
    pub(self) fn get_connect(&self) -> &C::Connect
    where
        C: GetDatabaseConnect,
    {
        self.0.get_connect()
    }

    pub(self) async fn get_transaction(
        &'c self,
    ) -> Result<C::Transaction<'c>, C::Error>
    where
        C: GetDatabaseTransaction,
    {
        self.0.get_transaction().await
    }
}

impl<'c, C> SubOperate<'c> for VideoOperate<'c, C>
where
    C: GetDatabaseConnect ,
{
    type Parent= SqlCeobeOperation<'c, C>;

    fn from_parent(parent: &'c Self::Parent) -> Self {
        Self(parent.0)
    }
}

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;
