use axum::extract::rejection::QueryRejection;
use mongo_migration::mongo_models::ceobe_operation::plugin_version;
use orm_migrate::sql_models::ceobe_operation::app_version;
use resp_result::FlagRespResult;
use ceobe_operate;
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
