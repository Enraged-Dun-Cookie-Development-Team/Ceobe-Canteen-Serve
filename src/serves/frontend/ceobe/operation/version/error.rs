use axum::extract::rejection::QueryRejection;
use persistence::{
    ceobe_operate,
    ceobe_operate::models::{app_version, plugin_version},
};
use resp_result::FlagRespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationVersionError

    AppCheck = app_version::CheckError
    PluginCheck = plugin_version::CheckError
    Query = QueryRejection
    DbOperate = ceobe_operate::app_version::OperateError
    MongoDbError = ceobe_operate::plugin_version::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagVersionRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationVersionError>;
