use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use fetcher_logic::error::LogicError;
use orm_migrate::sql_models::fetcher::{
    datasource_config::{
        checkers::CheckError, operate::OperateError as DatasourceOperateError,
    },
    platform_config::operate::OperateError as PlatformOperateError,
};
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub DatasourceConfigError

    Json = JsonRejection
    Query = QueryRejection
    DatasourceOperate = DatasourceOperateError
    PlatformOperate = PlatformOperateError
    Check = CheckError
    PageSize = NonZeroUnsignedError
    Logic = LogicError
}

pub type DatasourceConfigRResult<T> = RespResult<T, DatasourceConfigError>;
