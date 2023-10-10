use ceobe_cookie_logic::error::LogicError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CookieNewestError

    LogicError = LogicError
}

pub type CookieNewestRResult<T> = RespResult<T, CookieNewestError>;
