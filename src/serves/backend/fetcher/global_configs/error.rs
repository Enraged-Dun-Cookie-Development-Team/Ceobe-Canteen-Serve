use axum::extract::rejection::JsonRejection;
use axum_resp_result::RespResult;
use fetcher_logic::error::LogicError;
use persistence::fetcher::models::global_config::checkers::CheckError;

use crate::error_generate;

error_generate! {
    pub GlobalConfigError

    Json = JsonRejection
    Check = CheckError
    Logic = LogicError
}

pub type GlobalConfigRResult<T> = RespResult<T, GlobalConfigError>;
