use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::announcement;
use resp_result::{FlagRespResult, RespResult};

use crate::error_generate;

error_generate! {
    pub CdnOperationAnnouncementError

    Logic = LogicError
}

pub(super) type AnnouncementRespResult<T> =
    RespResult<T, CdnOperationAnnouncementError>;
