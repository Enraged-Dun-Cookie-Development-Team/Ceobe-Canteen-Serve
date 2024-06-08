use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::video;
use resp_result::{FlagRespResult, RespResult};

use crate::error_generate;

error_generate! {
    pub CdnOperationVideoError

    Logic = LogicError
}

pub(super) type VideoRespResult<T> =
    RespResult<T, CdnOperationVideoError>;
