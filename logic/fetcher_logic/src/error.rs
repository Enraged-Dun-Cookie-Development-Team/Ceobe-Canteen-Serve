use persistence::fetcher::{
    config::OperateError as FetcherConfigOperateError,
    datasource_combination::OperateError as DatasourceCombinationOperateError,
    datasource_config::OperateError as DatasourceConfigOperateError,
    global_config::OperateError as GlobalConfigOperateError,
    platform_config::OperateError as PlatformConfigOperateError,
};
use redis::RedisError;
use persistence::{
    fetcher::models::{
        config::checkers::CheckError as FetcherConfigCheckError,
        datasource_config::checkers::CheckError as DatasourceConfigCheckError,
        global_config::checkers::CheckError as GlobalConfigCheckError,
        platform_config::checkers::CheckError as PlatformConfigCheckError,
    },
    prelude::sea_orm,
};
use status_err::{ErrPrefix, StatusErr};
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
    DatasourceCombinationOperateError(
        #[from] DatasourceCombinationOperateError,
    ),

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

    #[error("该平台不存在")]
    #[status_err(err(err_code = 0x00_13, prefix = "ErrPrefix::CHECKER"))]
    PlatformNotFound,

    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("Redis异常: {0}")]
    Redis(#[from] RedisError),

    #[error("Json 反/序列化失败 {0}")]
    Json(#[from] serde_json::Error),
    #[error("Platform 不一致")]
    #[status_err(err(err_code = 0x00_16u16, prefix = "ErrPrefix::CHECKER"))]
    PlatFromNotSame,
}

#[allow(dead_code)]
pub(crate) type LogicResult<T> = Result<T, LogicError>;
