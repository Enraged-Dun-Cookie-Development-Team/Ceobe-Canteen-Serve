use std::{convert::Infallible, string::FromUtf8Error};

use persistence::ceobe_operate::{
    announcement::OperateError as AnnouncementOperateError,
    models::tool_link::CheckError,
    resource::OperateError as ResourceOperateError,
    tool_link::OperateError as ToolLinkIOperateError,
    tool_link_mongodb::OperateError,
    video::OperateError as VideoOperateError,
};
use request_clients::error::ChannelClose;
use status_err::StatusErr;
use tencent_cloud_server::error::TcCloudError;
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
    ResourceOperateError(#[from] ResourceOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    Utf8(#[from] FromUtf8Error),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    ChannelClose(#[from] ChannelClose),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    VideoOperateError(#[from] VideoOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    TcCloudError(#[from] TcCloudError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    MongoError(#[from] OperateError),
}

impl From<Infallible> for LogicError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
