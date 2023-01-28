use resp_result::FlagRespResult;
use ceobe_operate::video;
use crate::error_generate;

error_generate! {
    pub CeobeOperationVideoError

    DbOperate = video::OperateError
    ModifyVerify = modify_cache::Error
}

pub(super) type FlagVideoRespResult<T> =
    FlagRespResult<Option<T>, CeobeOperationVideoError>;
