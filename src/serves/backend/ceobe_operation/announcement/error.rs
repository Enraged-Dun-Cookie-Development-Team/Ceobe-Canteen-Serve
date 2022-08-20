use axum::extract::rejection::{JsonRejection, QueryRejection};
use orm_migrate::sql_models::ceobe_operation::announcement;
use resp_result::RespResult;

use crate::{error_generate, utils::user_authorize::error::AuthError};

error_generate! {
    pub CeobeOperationAnnouncementError

    Auth = AuthError
    Json = JsonRejection
    Query = QueryRejection
    Check = announcement::checkers::CheckError
    DbOperate = announcement::operate::OperateError
}

pub(crate) type AnnouncementRespResult<T> =
    RespResult<T, CeobeOperationAnnouncementError>;
