use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::video;

use crate::{error_generate, utils::user_authorize::error::AuthError};

error_generate! {
    pub CeobeOperationVideoError

    Auth = AuthError
    Url = url::ParseError
    Json = JsonRejection
    Query = QueryRejection
    Check = video::CheckError
    Logic = LogicError
}

pub(super) type VideoRespResult<T> = RespResult<T, CeobeOperationVideoError>;
