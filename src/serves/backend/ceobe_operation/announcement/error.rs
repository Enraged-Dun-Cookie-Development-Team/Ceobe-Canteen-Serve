use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::announcement;

use crate::{error_generate, utils::user_authorize::error::AuthError};

error_generate! {
    pub CeobeOperationAnnouncementError

    Auth = AuthError
    Json = JsonRejection
    Query = QueryRejection
    Check = announcement::CheckError
    Logic = LogicError
}

pub(crate) type AnnouncementRespResult<T> =
    RespResult<T, CeobeOperationAnnouncementError>;
