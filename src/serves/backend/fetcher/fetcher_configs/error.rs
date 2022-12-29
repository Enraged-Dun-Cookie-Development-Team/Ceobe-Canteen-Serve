

use axum::extract::rejection::{JsonRejection, QueryRejection};
use orm_migrate::sql_models::fetcher::config::operate::OperateError;
use resp_result::RespResult;
use fetcher_logic::error::LogicError;

use crate::error_generate;

error_generate! {
    pub FetcherConfigError

    Json = JsonRejection
    Query = QueryRejection
    Operate = OperateError
    Logic = LogicError
}

pub type FetcherConfigRResult<T> = RespResult<T, FetcherConfigError>;