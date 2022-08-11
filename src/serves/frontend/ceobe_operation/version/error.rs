
use axum::extract::rejection::QueryRejection;
use crate::models::mongo::plugin_version::{self, check::CheckError as PluginCheckError};
use crate::models::sql::app_version::{self, checkers::CheckError as AppCheckError};
use resp_result::RespResult;

use crate::{error_generate};

error_generate! {
    pub CeobeOperationVersionError

    AppCheck = AppCheckError
    PluginCheck = PluginCheckError
    Query = QueryRejection
    DbOperate = app_version::operate::OperateError
    MongoDbError = plugin_version::operates::OperateError
}

pub(super) type VersionRespResult<T> = RespResult<T, CeobeOperationVersionError>;