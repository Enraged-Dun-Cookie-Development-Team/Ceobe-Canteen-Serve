use axum::extract::rejection::JsonRejection;
use persistence::ceobe_operate::{app_version, release_version};
use axum_resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationAppVersionError

    Json = JsonRejection
    Check = app_version::CheckError
    DbOperate = app_version::OperateError
    ReleaseDbError = release_version::Error
}

pub(super) type AppRespResult<T> =
    RespResult<T, CeobeOperationAppVersionError>;
