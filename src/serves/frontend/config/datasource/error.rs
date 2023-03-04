use axum::extract::rejection::QueryRejection;
use resp_result::RespResult;
use fetcher::datasource_config::OperateError as DatasourceOperateError;

use crate::error_generate;

error_generate! {
    pub DatasourceError

    Query = QueryRejection
    DatasourceOperate = DatasourceOperateError
}

pub type DatasourceRResult<T> = RespResult<T, DatasourceError>;
