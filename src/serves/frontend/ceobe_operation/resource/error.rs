use resp_result::FlagRespResult;

use crate::{
    error_generate,
    models::sql::resource::{checkers::CheckError, operate::OperateError},
};

error_generate! {
    pub ResourceError

    Check = CheckError
    DbOperate = OperateError
    ModifyVerify = modify_cache::Error
}

pub type FlagResourceRespResult<T> = FlagRespResult<Option<T>, ResourceError>;
