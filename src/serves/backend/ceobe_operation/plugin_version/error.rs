use axum::extract::rejection::JsonRejection;
use mongo_migration::mongo_models::ceobe_operation::plugin_version;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationPluginVersionError

    Json = JsonRejection
    Check = plugin_version::check::CheckError
    DbOperate = plugin_version::operates::OperateError
}

pub(super) type PluginRespResult<T> =
    RespResult<T, CeobeOperationPluginVersionError>;
