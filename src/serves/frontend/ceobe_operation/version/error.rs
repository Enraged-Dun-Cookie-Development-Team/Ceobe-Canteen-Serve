use axum::extract::rejection::QueryRejection;
use mongo_migration::mongo_models::ceobe_operation::plugin_version;
use orm_migrate::sql_models::ceobe_operation::app_version;
use resp_result::FlagRespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationVersionError

    AppCheck = app_version::CheckError
    PluginCheck = plugin_version::check::CheckError
    Query = QueryRejection
    DbOperate = app_version::OperateError
    MongoDbError = plugin_version::operates::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagVersionRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationVersionError>;
