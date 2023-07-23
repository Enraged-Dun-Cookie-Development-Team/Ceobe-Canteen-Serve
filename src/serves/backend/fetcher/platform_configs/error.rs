use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use persistence::fetcher::platform_config::OperateError;
use fetcher_logic::error::LogicError;
use persistence::fetcher::models::platform_config;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub PlatformConfigError

    Json = JsonRejection
    Logic = LogicError
    Query = QueryRejection
    Check = platform_config::checkers::CheckError
    DbOperate = OperateError
    PageSize = NonZeroUnsignedError
}

pub type PlatformConfigRResult<T> = RespResult<T, PlatformConfigError>;
