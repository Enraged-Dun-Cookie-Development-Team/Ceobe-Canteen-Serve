use db_ops_prelude::{sea_orm, ErrPrefix, StatusErr, ThisError};

pub use crate::common::tool_link::ToolLinkOperate;

mod create;
mod delete;
mod retrieve;
mod update;
mod verify;

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("工具链接 {0} 不存在")]
    #[status_err(err(
        err_code = 0x000A,
        prefix = "ErrPrefix::NOT_FOUND",
        resp_msg = "蹲饼器数据源不存在"
    ))]
    ToolLinkNotFound(i32),
}
type OperateResult<T> = Result<T, OperateError>;
