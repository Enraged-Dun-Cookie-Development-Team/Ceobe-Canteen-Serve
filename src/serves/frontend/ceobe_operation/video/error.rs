use crate::models::sql::video;
use resp_result::{RespResult, FlagRespResult};

use crate::error_generate;

error_generate! {
    pub CeobeOperationVideoError

    DbOperate = video::operate::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagVideoRespResult<T> = FlagRespResult<Option<T>, CeobeOperationVideoError>;
