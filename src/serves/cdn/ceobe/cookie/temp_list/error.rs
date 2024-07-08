use axum::extract::rejection::QueryRejection;
use ceobe_cookie_logic::error::LogicError;
use persistence::ceobe_cookie::temp_list::OperateError as TempListOperateError;
use axum_resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeCookieTempListError

    Query = QueryRejection
    Logic = LogicError
    CookieTempListOperate = TempListOperateError
}

pub type CeobeCookieRResult<T> = RespResult<T, CeobeCookieTempListError>;
