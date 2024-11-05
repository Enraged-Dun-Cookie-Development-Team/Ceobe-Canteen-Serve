use axum::extract::rejection::QueryRejection;
use axum_resp_result::RespResult;
use ceobe_cookie_logic::error::LogicError;
use persistence::ceobe_cookie::terra_comic::OperateError as TerraComicOperateError;

use crate::error_generate;

error_generate! {
    pub CeobeCookieTerraComicError

    Query = QueryRejection
    Logic = LogicError
    TerraComicOperateError = TerraComicOperateError
}

pub type CeobeCookieRResult<T> = RespResult<T, CeobeCookieTerraComicError>;
