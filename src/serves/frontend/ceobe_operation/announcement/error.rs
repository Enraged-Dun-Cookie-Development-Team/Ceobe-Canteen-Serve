use orm_migrate::sql_models::ceobe_operation::announcement;
use resp_result::RespResult;

use crate::{error_generate};

error_generate! {
    pub CeobeOperationAnnouncementError

    DbOperate = announcement::operate::OperateError
}

pub(super) type AnnouncementRespResult<T> = RespResult<T, CeobeOperationAnnouncementError>;
