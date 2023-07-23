use axum::extract::rejection::QueryRejection;
use persistence::ceobe_cookie::terra_comic::OperateError as TerraComicOperateError;
use ceobe_cookie_logic::error::LogicError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeCookieTerraComicError

    Query = QueryRejection
    Logic = LogicError
    TerraComicOperateError = TerraComicOperateError
}

pub type CeobeCookieRResult<T> = RespResult<T, CeobeCookieTerraComicError>;
