use axum::extract::rejection::JsonRejection;
use resp_result::RespResult;
use ceobe_operate::plugin_version::{OperateError,CheckError};
use crate::error_generate;

error_generate! {
    pub CeobeOperationPluginVersionError

    Json = JsonRejection
    Check = CheckError
    DbOperate = OperateError
}

pub(super) type PluginRespResult<T> =
    RespResult<T, CeobeOperationPluginVersionError>;
