use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;

use crate::error_generate;

error_generate! {
    pub OperateToolLinkError

    LogicError = LogicError
}

pub type OperateToolLinkRResult<T> = RespResult<T, OperateToolLinkError>;
