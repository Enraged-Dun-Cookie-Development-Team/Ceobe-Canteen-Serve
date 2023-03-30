use axum::extract::rejection::{JsonRejection, QueryRejection};
use ceobe_cookie_logic::error::LogicError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub AnalyzeCookieError

    Json = JsonRejection
    LogicError = LogicError
}

pub type AnalyzeCookieRResult<T> = RespResult<T, AnalyzeCookieError>;
