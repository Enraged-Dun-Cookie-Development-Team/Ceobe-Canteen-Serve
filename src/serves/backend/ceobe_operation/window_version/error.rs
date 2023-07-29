use axum::extract::rejection::JsonRejection;
use persistence::ceobe_operate::window_version;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationWindowVersionError

    Json = JsonRejection
    Check = window_version::CheckError
    DbOperate = window_version::OperateError
}

pub(super) type WindowRespResult<T> =
    RespResult<T, CeobeOperationWindowVersionError>;
