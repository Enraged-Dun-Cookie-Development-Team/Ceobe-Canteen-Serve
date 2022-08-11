use axum::extract::rejection::QueryRejection;
use resp_result::RespResult;

use crate::{
    error_generate,
    models::{
        mongo::plugin_version::{
            self, check::CheckError as PluginCheckError,
        },
        sql::app_version::{self, checkers::CheckError as AppCheckError},
    },
};

error_generate! {
    pub CeobeOperationVersionError

    AppCheck = AppCheckError
    PluginCheck = PluginCheckError
    Query = QueryRejection
    DbOperate = app_version::operate::OperateError
    MongoDbError = plugin_version::operates::OperateError
}

pub(super) type VersionRespResult<T> =
    RespResult<T, CeobeOperationVersionError>;
