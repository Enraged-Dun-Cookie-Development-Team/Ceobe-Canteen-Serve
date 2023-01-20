pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod verify;

use sea_orm::FromQueryResult;
use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

pub struct FetcherDatasourceConfigSqlOperate;

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
