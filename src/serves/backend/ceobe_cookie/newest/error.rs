use axum_resp_result::RespResult;
use ceobe_cookie_logic::error::LogicError;

use crate::error_generate;

error_generate! {
    pub CookieNewestError

    LogicError = LogicError
}

pub type CookieNewestRResult<T> = RespResult<T, CookieNewestError>;
