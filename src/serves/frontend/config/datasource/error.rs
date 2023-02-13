use axum::extract::rejection::QueryRejection;
use resp_result::RespResult;

use crate::error_generate;
use orm_migrate::sql_models::fetcher::{
    datasource_config::{operate::OperateError as DatasourceOperateError}
};

error_generate! {
    pub DatasourceError

    Query = QueryRejection
    DatasourceOperate = DatasourceOperateError
}

pub type DatasourceRResult<T> = RespResult<T, DatasourceError>;