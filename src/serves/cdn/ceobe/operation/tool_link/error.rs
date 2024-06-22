use axum::extract::rejection::QueryRejection;
use resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;

use crate::error_generate;

error_generate!{
    pub CeobeOperateToolLinkError

    Query = QueryRejection
    Logic = LogicError
}

pub type CeobeToolLinkRResult<T> = RespResult<T, CeobeOperateToolLinkError>;