use axum::extract::rejection::{JsonRejection, QueryRejection};
use fetcher::config::OperateError;
use fetcher_logic::error::LogicError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub FetcherConfigError

    Json = JsonRejection
    Query = QueryRejection
    Operate = OperateError
    Logic = LogicError
}

pub type FetcherConfigRResult<T> = RespResult<T, FetcherConfigError>;
