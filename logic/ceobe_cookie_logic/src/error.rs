use std::convert::Infallible;

use bitmap_convert::error::Error as BitmapConvError;
use ceobe_cookie::{
    analyze::OperateError as AnalyzeOperateError,
    temp_list::OperateError as TempListOperateError,
};
use ceobe_user::property::OperateError as CeobeUserOperateError;
use fetcher::{
    datasource_combination::OperateError as DatasourceCombinationOperateError,
    datasource_config::OperateError as DatasourceOperateError,
};
use mob_push_server::MobPushError;
use redis::RedisError;
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    TempListOperateError(#[from] TempListOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    AnalyzeOperateError(#[from] AnalyzeOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    BitmapConvError(#[from] BitmapConvError),

    #[error("Json 反/序列化失败 {0}")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    CeobeUserOperateError(#[from] CeobeUserOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceOperateError(#[from] DatasourceOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceCombinationOperateError(
        #[from] DatasourceCombinationOperateError,
    ),

    #[error(transparent)]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x001B,
        http_code = "HttpCode::CONFLICT"
    ))]
    MobPushError(#[from] MobPushError),

    #[error(transparent)]
    QqChannelError(#[from] qq_channel_warning::Error),

    #[error(transparent)]
    #[status_err(err(prefix = "ErrPrefix::SERVE", err_code = 0x0003,))]
    JoinError(#[from] JoinError),

    #[error("Redis异常: {0}")]
    Redis(#[from] RedisError),

    #[error("更新饼id缓存失效：{0}")]
    #[status_err(err(prefix = "ErrPrefix::CHECKER", err_code = 0x001D,))]
    UpdateCookieCacheExpire(String),
}

impl From<Infallible> for LogicError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
