mod delete;
mod retrieve;
mod update;

use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::{GetDatabaseConnect, GetDatabaseTransaction},
};
use status_err::StatusErr;
use thiserror::Error;

pub struct AnnouncementOperate<'c, C>(&'c C);

impl<'c, C> AnnouncementOperate<'c, C> {
    pub(self) fn get_connect(&self) -> &C::Connect<'_>
    where
        C: GetDatabaseConnect,
    {
        self.0.get_connect()
    }

    pub(self) async fn get_transaction(
        &self,
    ) -> Result<C::Transaction<'_>, C::Error>
    where
        C: GetDatabaseTransaction,
    {
        self.0.get_transaction().await
    }
}

impl<'c, C> SubOperate<'c> for AnnouncementOperate<'c, C>
where
    C: GetDatabaseConnect,
{
    type Parent = SqlCeobeOperation<'c, C>;

    fn from_parent(parent: &'c mut Self::Parent) -> Self {
        Self(parent.0)
    }
}

use crate::ceobe_operation::SqlCeobeOperation;

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;
