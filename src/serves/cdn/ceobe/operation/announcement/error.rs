use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;

use crate::error_generate;

error_generate! {
    pub CdnOperationAnnouncementError

    Logic = LogicError
}

pub(super) type AnnouncementRespResult<T> =
    RespResult<T, CdnOperationAnnouncementError>;
