use axum::extract::rejection::QueryRejection;
use mongo_migration::mongo_models::ceobe_operation::plugin_version;
use orm_migrate::sql_models::ceobe_operation::app_version::{
    self, checkers::CheckError,
};
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationVersionError

    Check = CheckError
    Query = QueryRejection
    DbOperate = app_version::operate::OperateError
    MongoDbError = plugin_version::operates::OperateError
}

pub(super) type VersionRespResult<T> =
    RespResult<T, CeobeOperationVersionError>;
