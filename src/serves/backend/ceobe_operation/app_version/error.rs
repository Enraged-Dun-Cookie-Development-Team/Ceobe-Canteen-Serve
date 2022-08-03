use axum::extract::rejection::JsonRejection;
use orm_migrate::sql_models::ceobe_operation::app_version;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationAppVersionError

    Json = JsonRejection
    Check = app_version::checkers::CheckError
    DbOperate = app_version::operate::OperateError
}

pub(super) type AppRespResult<T> =
    RespResult<T, CeobeOperationAppVersionError>;