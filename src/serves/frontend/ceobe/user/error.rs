use axum::extract::rejection::{JsonRejection, QueryRejection};
use ceobe_user_logic::error::LogicError;
use persistence::ceobe_user::{
    models::check::CheckError as CeobeUserPropertyCheckError,
    property::OperateError,
};
use axum_resp_result::RespResult;

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
