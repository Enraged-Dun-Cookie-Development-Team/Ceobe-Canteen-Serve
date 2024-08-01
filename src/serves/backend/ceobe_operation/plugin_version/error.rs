use axum::extract::rejection::JsonRejection;
use axum_resp_result::RespResult;
use persistence::ceobe_operate::{
    plugin_version::{CheckError, OperateError},
    release_version,
};

use crate::error_generate;

error_generate! {
    pub CeobeOperationPluginVersionError

    Json = JsonRejection
    Check = CheckError
    DbOperate = OperateError
    ReleaseDbOperate = release_version::Error
}

pub(super) type PluginRespResult<T> =
    RespResult<T, CeobeOperationPluginVersionError>;
