use axum::extract::rejection::QueryRejection;
use resp_result::RespResult;

use crate::{error_generate, serves::backend::ceobe_operation::announcement};

error_generate! {
    pub CeobeOperationAnnouncementError

    Query = QueryRejection
    DbOperate = announcement::operate::OperateError
}

pub(super) type VideoRespResult<T> = RespResult<T, CeobeOperationAnnouncementError>;
