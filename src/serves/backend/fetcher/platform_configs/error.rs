use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::{prefabs::num_check::NonZeroUnsignedError, QueryCheckExtract};
use fetcher_logic::error::LogicError;
use orm_migrate::sql_models::fetcher::platform_config;
use page_size::request::PageSizeChecker;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub PlatformConfigError

    Json = JsonRejection
    Logic = LogicError
    Query = QueryRejection
    Check = platform_config::checkers::CheckError
    DbOperate = platform_config::operate::OperateError
    PageSize = NonZeroUnsignedError
}

pub type PlatformConfigRResult<T> = RespResult<T, PlatformConfigError>;