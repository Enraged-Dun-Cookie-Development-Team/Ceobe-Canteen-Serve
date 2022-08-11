use axum::extract::rejection::QueryRejection;
use resp_result::RespResult;

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
}

pub(super) type VersionRespResult<T> =
    RespResult<T, CeobeOperationVersionError>;
