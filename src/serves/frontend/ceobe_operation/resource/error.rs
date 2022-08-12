use resp_result::FlagRespResult;

use crate::models::sql::resource::operate::OperateError;
use crate::models::sql::resource::checkers::CheckError;
use crate::error_generate;

error_generate! {
    pub ResourceError

    Check = CheckError
    DbOperate = OperateError
    ModifyVerify = modify_cache::Error
}

pub type FlagResourceRespResult<T> = FlagRespResult<Option<T>, ResourceError>;