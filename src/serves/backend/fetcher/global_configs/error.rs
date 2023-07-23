use axum::extract::rejection::JsonRejection;
use fetcher_logic::error::LogicError;
use persistence::models::mysql::fetcher::global_config::checkers::CheckError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub GlobalConfigError

    Json = JsonRejection
    Check = CheckError
    Logic = LogicError
}

pub type GlobalConfigRResult<T> = RespResult<T, GlobalConfigError>;
