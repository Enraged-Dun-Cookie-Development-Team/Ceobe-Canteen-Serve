use std::convert::Infallible;

use persistence::ceobe_operate::{
    models::tool_link::CheckError,
    tool_link::OperateError as ToolLinkIOperateError,
    announcement::OperateError as  AnnouncementOperateError,
    resource::OperateError as  ResourceOperateError
};
use status_err::StatusErr;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error("Json 反/序列化失败 {0}")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    ToolLinkIOperateError(#[from] ToolLinkIOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    ToolLinkCheckError(#[from] CheckError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    AnnouncementOperateError(#[from] AnnouncementOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    ResourceOperateError(#[from] ResourceOperateError)
}

impl From<Infallible> for LogicError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
