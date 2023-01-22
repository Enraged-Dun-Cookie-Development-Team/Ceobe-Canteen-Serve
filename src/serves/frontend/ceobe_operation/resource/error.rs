use orm_migrate::sql_models::ceobe_operation::resource;
use resp_result::FlagRespResult;

use crate::error_generate;

error_generate! {
    pub ResourceError

    Check = resource::CheckError
    DbOperate = resource::OperateError
    ModifyVerify = modify_cache::Error
}

pub type FlagResourceRespResult<T> = FlagRespResult<Option<T>, ResourceError>;
