mod create;
mod delete;
mod retrieve;
mod update;

use sql_connection::database_traits::{
    database_operates::sub_operate::SubOperate,
    get_connect::{GetDatabaseConnect, GetDatabaseTransaction},
};
use status_err::StatusErr;
use thiserror::Error;

pub struct ResourceOperate<'op, C: 'op>(&'op C);

impl<'op, C: 'op> ResourceOperate<'op, C> {
    pub(self) async fn get_transaction(
        &'op self,
    ) -> Result<C::Transaction<'op>, C::Error>
    where
        C: GetDatabaseTransaction,
    {
        self.0.get_transaction().await
    }
}

impl<'op, C> SubOperate<'op> for ResourceOperate<'op, C>
where
    C: 'op + GetDatabaseConnect,
{
    type Parent = SqlCeobeOperation<'op, C>;

    fn from_parent(parent: &'op Self::Parent) -> Self { Self(parent.0) }
}

use status_err::{ErrPrefix, HttpCode};

use crate::ceobe_operation::SqlCeobeOperation;
#[derive(Debug, Error, StatusErr)]
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
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;
