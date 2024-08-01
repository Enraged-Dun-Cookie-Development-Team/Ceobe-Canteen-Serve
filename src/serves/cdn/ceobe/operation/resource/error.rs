use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::resource;

use crate::error_generate;

error_generate! {
    pub ResourceError

    Check = resource::CheckError
    Logic = LogicError
}

pub type ResourceRespResult<T> = RespResult<T, ResourceError>;
