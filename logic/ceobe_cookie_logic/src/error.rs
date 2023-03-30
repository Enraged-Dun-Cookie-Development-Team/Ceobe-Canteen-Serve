use bitmap_convert::error::Error as BitmapConvError;
use ceobe_cookie::temp_list::OperateError as TempListOperateError;
use mob_push_server::MobPushError;
use status_err::{StatusErr, HttpCode, ErrPrefix};
use thiserror::Error;
use ceobe_user::property::OperateError as CeobeUserOperateError;
use fetcher::{
    datasource_config::OperateError as DatasourceOperateError,
    datasource_combination::OperateError as DatasourceCombinationOperateError
};

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

    #[error(transparent)]
    #[status_err(err = "transparent")]
    CeobeUserOperateError(#[from] CeobeUserOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceOperateError(#[from] DatasourceOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceCombinationOperateError(#[from] DatasourceCombinationOperateError),

    #[error(transparent)]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0008,
        http_code = "HttpCode::CONFLICT"
    ))]
    MobPushError(#[from] MobPushError),
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
