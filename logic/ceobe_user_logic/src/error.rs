use ceobe_user::user::OperateError as CeobeUserOperateError;
use mongo_models::{
    ceobe::user::{
        check::CheckError as CeobeUserCheckError,
        
    },
    mongo_connection::MongoDbError,
};
use sql_models::{
    fetcher::datasource_config::{
        checkers::CheckError as DatasourceConfigCheckError,
        operate::OperateError as DatasourceConfigOperateError,
    },
    sql_connection::sea_orm,
};
use status_err::StatusErr;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceConfigOperateError(#[from] DatasourceConfigOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    DatasourceConfigCheckError(#[from] DatasourceConfigCheckError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    CeobeUserOperateError(#[from] CeobeUserOperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    CeobeUserCheckError(#[from] CeobeUserCheckError),

    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("Mongo异常: {0}")]
    Mongo(#[from] MongoDbError),
}

#[allow(dead_code)]
pub(crate) type LogicResult<T> = Result<T, LogicError>;
