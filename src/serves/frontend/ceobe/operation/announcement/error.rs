use axum_resp_result::FlagRespResult;
use persistence::ceobe_operate::announcement;

use crate::error_generate;

error_generate! {
    pub CeobeOperationAnnouncementError

    DbOperate = announcement::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagAnnouncementRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationAnnouncementError>;
