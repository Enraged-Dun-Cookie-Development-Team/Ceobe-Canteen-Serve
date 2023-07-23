use axum::extract::rejection::{JsonRejection, QueryRejection};
use persistence::ceobe_operate::announcement;
use resp_result::RespResult;

use crate::{error_generate, utils::user_authorize::error::AuthError};

error_generate! {
    pub CeobeOperationAnnouncementError

    Auth = AuthError
    Json = JsonRejection
    Query = QueryRejection
    Check = announcement::CheckError
    DbOperate = announcement::OperateError
}

pub(crate) type AnnouncementRespResult<T> =
    RespResult<T, CeobeOperationAnnouncementError>;
