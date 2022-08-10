use axum::extract::rejection::JsonRejection;
use orm_migrate::sql_models::ceobe_operation::resource::{
    checkers::CheckError, operate::OperateError,
};
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub ResourceError

    Json = JsonRejection
    Check = CheckError
    DbOperate = OperateError
    ModifyCheck = modify_cache::Error
}

pub type ResourceRResult<T> = RespResult<T, ResourceError>;
