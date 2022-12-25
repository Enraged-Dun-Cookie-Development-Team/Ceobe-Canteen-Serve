
use sql_models::fetcher::global_config::{operate::OperateError as GlobalConfigOperateError, checkers::CheckError as GlobalConfigCheckError};
use sql_models::fetcher::datasource_config::{operate::OperateError as DatasourceConfigOperateError, checkers::CheckError as DatasourceConfigCheckError};
use sql_models::fetcher::platform_config::{operate::OperateError as PlatformConfigOperateError, checkers::CheckError as PlatformConfigCheckError};
use sql_models::fetcher::config::{operate::OperateError as FetcherConfigOperateError, checkers::CheckError as FetcherConfigCheckError};
use status_err::StatusErr;
use thiserror::Error;


#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    GlobalConfigOperateError(#[from] GlobalConfigOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceConfigOperateError(#[from] DatasourceConfigOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    PlatformConfigOperateError(#[from] PlatformConfigOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    FetcherConfigOperateError(#[from] FetcherConfigOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    GlobalConfigCheckError(#[from] GlobalConfigCheckError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceConfigCheckError(#[from] DatasourceConfigCheckError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    PlatformConfigCheckError(#[from] PlatformConfigCheckError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    FetcherConfigCheckError(#[from] FetcherConfigCheckError),
}

#[allow(dead_code)]
pub(crate) type LogicResult<T> = Result<T, LogicError>;