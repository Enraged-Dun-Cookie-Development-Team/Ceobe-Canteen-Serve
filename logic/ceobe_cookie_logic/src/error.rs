use bitmap_convert::error::Error as BitmapConvError;
use ceobe_cookie::temp_list::OperateError as TempListOperateError;
use status_err::StatusErr;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    TempListOperateError(#[from] TempListOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    BitmapConvError(#[from] BitmapConvError),

    #[error("Json 反/序列化失败 {0}")]
    Json(#[from] serde_json::Error),
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
