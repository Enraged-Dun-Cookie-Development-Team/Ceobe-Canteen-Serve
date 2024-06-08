use axum::extract::rejection::JsonRejection;
use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::resource::{CheckError, OperateError};
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub ResourceError

    Json = JsonRejection
    Check = CheckError
    Logic = LogicError
}

pub type ResourceRResult<T> = RespResult<T, ResourceError>;
