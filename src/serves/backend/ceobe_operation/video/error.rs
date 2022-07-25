use std::string::FromUtf8Error;

use axum::extract::rejection::{JsonRejection, QueryRejection};
use orm_migrate::sql_models::ceobe_operation::video;
use resp_result::RespResult;

use crate::{error_generate, utils::user_authorize::error::AuthError};

error_generate! {
    pub CeobeOperationVideoError

    Auth = AuthError
    Request = reqwest::Error
    Utf8 = FromUtf8Error
    Url = url::ParseError
    Json = JsonRejection
    Query = QueryRejection
    Check = video::checkers::CheckError
    DbOperate = video::operate::OperateError
}

pub(super) type VideoRespResult<T> = RespResult<T, CeobeOperationVideoError>;
