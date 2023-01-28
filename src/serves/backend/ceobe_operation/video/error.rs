use std::string::FromUtf8Error;

use axum::extract::rejection::{JsonRejection, QueryRejection};
use resp_result::RespResult;
use ceobe_operate::video;
use crate::{error_generate, utils::user_authorize::error::AuthError};

error_generate! {
    pub CeobeOperationVideoError

    Auth = AuthError
    Request = reqwest::Error
    Utf8 = FromUtf8Error
    Url = url::ParseError
    Json = JsonRejection
    Query = QueryRejection
    Check = video::CheckError
    DbOperate = video::OperateError
}

pub(super) type VideoRespResult<T> = RespResult<T, CeobeOperationVideoError>;
