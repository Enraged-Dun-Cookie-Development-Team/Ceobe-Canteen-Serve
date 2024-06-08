use ceobe_operation_logic::error::LogicError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CdnOperationVideoError

    Logic = LogicError
}

pub(super) type VideoRespResult<T> = RespResult<T, CdnOperationVideoError>;
