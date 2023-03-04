use axum::extract::rejection::QueryRejection;
use fetcher::datasource_config::OperateError as DatasourceOperateError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub DatasourceError

    Query = QueryRejection
    DatasourceOperate = DatasourceOperateError
}

pub type DatasourceRResult<T> = RespResult<T, DatasourceError>;
