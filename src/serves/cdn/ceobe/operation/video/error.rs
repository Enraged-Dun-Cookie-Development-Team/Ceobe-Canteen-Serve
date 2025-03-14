use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;

use crate::error_generate;

error_generate! {
    pub CdnOperationVideoError

    Logic = LogicError
}

pub(super) type VideoRespResult<T> = RespResult<T, CdnOperationVideoError>;
