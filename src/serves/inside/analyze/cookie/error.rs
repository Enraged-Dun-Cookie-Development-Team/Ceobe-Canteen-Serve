use axum::extract::rejection::JsonRejection;
use axum_resp_result::RespResult;
use ceobe_cookie_logic::error::LogicError;

use crate::error_generate;

error_generate! {
    pub AnalyzeCookieError

    Json = JsonRejection
    LogicError = LogicError
}

pub type AnalyzeCookieRResult<T> = RespResult<T, AnalyzeCookieError>;
