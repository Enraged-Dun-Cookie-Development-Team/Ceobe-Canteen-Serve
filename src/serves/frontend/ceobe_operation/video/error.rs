use resp_result::FlagRespResult;

use crate::{error_generate, models::sql::video};

error_generate! {
    pub CeobeOperationVideoError

    DbOperate = video::operate::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagVideoRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationVideoError>;
