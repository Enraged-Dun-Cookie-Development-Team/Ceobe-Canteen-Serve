use bitmap_convert::error::Error as BitmapConvError;
use mob_push_server::MobPushError;
use persistence::{
    ceobe_cookie::analyze::OperateError as AnalyzeOperateError,
    ceobe_user::{
        models::check::CheckError as CeobeUserPropertyCheckError,
        property::OperateError as CeobeUserOperateError,
    },
    fetcher::{
        datasource_combination::OperateError as DatasourceCombinationOperateError,
        datasource_config::OperateError as DatasourceConfigOperateError,
        models::datasource_config::checkers::CheckError as DatasourceConfigCheckError,
    },
    help_crates::sea_orm,
    mongodb::MongoDbError,
};
use qiniu_service::error::ServiceError as QiniuServiceError;
use redis::RedisError;
use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
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
    DatasourceConfigCheckError(#[from] DatasourceConfigCheckError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    CeobeUserOperateError(#[from] CeobeUserOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    CeobeUserPropertyCheckerror(#[from] CeobeUserPropertyCheckError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    AnalyzeOperateError(#[from] AnalyzeOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    BitmapConvError(#[from] BitmapConvError),

    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("Mongo异常: {0}")]
    Mongo(#[from] MongoDbError),

    #[error(transparent)]
    #[status_err(err(prefix = "ErrPrefix::CHECKER", err_code = 0x001B,))]
    MobPushError(#[from] MobPushError),

    #[error("mobId不存在")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x001D,
        resp_msg = "注册失败，请联系开发者"
    ))]
    MobIdNotExist,

    #[error(transparent)]
    #[status_err(err = "transparent")]
    QiniuService(#[from] QiniuServiceError),

    #[error("数据源列表为空")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x001E,
        resp_msg = "数据源列表为空"
    ))]
    DatasourcesEmpty,

    #[error("Redis异常: {0}")]
    Redis(#[from] RedisError),

    #[error("Json 反/序列化失败 {0}")]
    Json(#[from] serde_json::Error),
}

#[allow(dead_code)]
pub(crate) type LogicResult<T> = Result<T, LogicError>;
