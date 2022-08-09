use axum::extract::rejection::QueryRejection;
use orm_migrate::sql_models::ceobe_operation::announcement;
use resp_result::RespResult;

use crate::{error_generate};

error_generate! {
    pub CeobeOperationAnnouncementError

    Query = QueryRejection
    DbOperate = announcement::operate::OperateError
}

pub(super) type AnnouncementRespResult<T> = RespResult<T, CeobeOperationAnnouncementError>;
