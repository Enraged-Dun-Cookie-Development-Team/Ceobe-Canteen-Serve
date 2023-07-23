use axum::extract::rejection::{JsonRejection, QueryRejection};
use persistence::ceobe_user::property::OperateError;
use ceobe_user_logic::error::LogicError;
use persistence::ceobe_user::models::check::CheckError as CeobeUserPropertyCheckError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeUserError

    Json = JsonRejection
    Query = QueryRejection
    Logic = LogicError
    Check = CeobeUserPropertyCheckError
    CeobeUserOperate = OperateError
}

pub type CeobeUserRResult<T> = RespResult<T, CeobeUserError>;
