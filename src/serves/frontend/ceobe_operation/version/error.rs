use axum::extract::rejection::QueryRejection;
use resp_result::{RespResult, FlagRespResult};

use crate::{
    error_generate,
    models::{mongo::plugin_version, sql::app_version},
};

error_generate! {
    pub CeobeOperationVersionError

    AppCheck = app_version::checkers::CheckError
    PluginCheck = plugin_version::check::CheckError
    Query = QueryRejection
    DbOperate = app_version::operate::OperateError
    MongoDbError = plugin_version::operates::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagVersionRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationVersionError>;
