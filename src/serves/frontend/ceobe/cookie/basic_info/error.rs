use ceobe_cookie_logic::error::LogicError;
use axum_resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeCookieInfoError

    Logic = LogicError
}

pub type CeobeCookieRResult<T> = RespResult<T, CeobeCookieInfoError>;
