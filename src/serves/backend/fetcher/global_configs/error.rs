use axum::extract::rejection::JsonRejection;
use fetcher_logic::error::LogicError;
use orm_migrate::sql_models::fetcher::global_config::{checkers::CheckError, operate::OperateError};
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub GlobalConfigError

    Json = JsonRejection
    Check = CheckError
    Logic = LogicError
}

pub type GlobalConfigRResult<T> = RespResult<T, GlobalConfigError>;