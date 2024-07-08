use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum_resp_result::RespResult;
use checker::prefabs::num_check::NonZeroUnsignedError;
use fetcher_logic::error::LogicError;
use persistence::fetcher::{
    models::platform_config, platform_config::OperateError,
};

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
