use axum::extract::rejection::QueryRejection;
use resp_result::RespResult;
use ceobe_cookie_logic::error::LogicError;
use ceobe_cookie::temp_list::OperateError as TempListOperateError;
use crate::error_generate;

error_generate! {
    pub CeobeCookieTempListError

    Query = QueryRejection
    Logic = LogicError
    CookieTempListOperate = TempListOperateError
}

pub type CeobeCookieRResult<T> = RespResult<T, CeobeCookieTempListError>;