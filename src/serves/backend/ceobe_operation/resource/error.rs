use axum::extract::rejection::JsonRejection;
use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::resource::CheckError;

use crate::error_generate;

error_generate! {
    pub ResourceError

    Json = JsonRejection
    Check = CheckError
    Logic = LogicError
}

pub type ResourceRResult<T> = RespResult<T, ResourceError>;
