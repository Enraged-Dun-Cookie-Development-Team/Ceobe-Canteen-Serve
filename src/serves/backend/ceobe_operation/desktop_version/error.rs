use axum::extract::rejection::JsonRejection;
use persistence::ceobe_operate::desktop_version;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationDesktopVersionError

    Json = JsonRejection
    Check = desktop_version::CheckError
    DbOperate = desktop_version::OperateError
}

pub(super) type DesktopRespResult<T> =
    RespResult<T, CeobeOperationDesktopVersionError>;
