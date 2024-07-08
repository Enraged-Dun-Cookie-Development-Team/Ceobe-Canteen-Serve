use axum::extract::rejection::QueryRejection;
use persistence::{
    ceobe_operate,
    ceobe_operate::{
        desktop_version,
        models::{app_version, plugin_version},
        release_version,
    },
};
use axum_resp_result::FlagRespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationVersionError

    AppCheck = app_version::CheckError
    PluginCheck = plugin_version::CheckError
    WindowCheck = desktop_version::CheckError
    Query = QueryRejection
    AppOperate = ceobe_operate::app_version::OperateError
    DesktopOperate = ceobe_operate::desktop_version::OperateError
    ReleaseDbOperate = release_version::Error
    PluginDbError = ceobe_operate::plugin_version::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagVersionRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationVersionError>;
