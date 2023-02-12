use axum::extract::rejection::QueryRejection;
use ceobe_user_logic::error::LogicError;
use resp_result::RespResult;
use axum::extract::rejection::JsonRejection;

use crate::error_generate;

error_generate! {
    pub CeobeUserError

    Json = JsonRejection
    Query = QueryRejection
    Logic = LogicError
}

pub type CeobeUserRResult<T> = RespResult<T, CeobeUserError>;