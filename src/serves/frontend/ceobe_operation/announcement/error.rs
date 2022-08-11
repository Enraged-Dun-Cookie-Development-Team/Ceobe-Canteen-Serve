use orm_migrate::sql_models::ceobe_operation::announcement;
use resp_result::FlagRespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationAnnouncementError

    DbOperate = announcement::operate::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagAnnouncementRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationAnnouncementError>;
