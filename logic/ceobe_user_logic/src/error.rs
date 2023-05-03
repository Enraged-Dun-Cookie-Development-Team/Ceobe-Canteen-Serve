use bitmap_convert::error::Error as BitmapConvError;
use ceobe_cookie::temp_list::OperateError as TemporaryListOperateError;
use ceobe_user::property::OperateError as CeobeUserOperateError;
use db_ops_prelude::{
    mongo_connection::MongoDbError,
    mongo_models::ceobe::user_property::check::CheckError as CeobeUserPropertyCheckerror,
    sea_orm,
    sql_models::fetcher::datasource_config::checkers::CheckError as DatasourceConfigCheckError,
};
use fetcher::{
    datasource_combination::OperateError as DatasourceCombinationOperateError,
    datasource_config::OperateError as DatasourceConfigOperateError,
};
use mob_push_server::MobPushError;
use qiniu_service::error::ServiceError as QiniuServiceError;
use status_err::{StatusErr, ErrPrefix, HttpCode};
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
    CeobeUserPropertyCheckerror(#[from] CeobeUserPropertyCheckerror),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    TemporaryListOperateError(#[from] TemporaryListOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    BitmapConvError(#[from] BitmapConvError),

    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("Mongo异常: {0}")]
    Mongo(#[from] MongoDbError),

    #[error(transparent)]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x001B,
    ))]
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
}

#[allow(dead_code)]
pub(crate) type LogicResult<T> = Result<T, LogicError>;
