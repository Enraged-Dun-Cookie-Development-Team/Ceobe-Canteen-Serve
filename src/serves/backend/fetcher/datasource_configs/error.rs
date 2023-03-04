use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use fetcher_logic::error::LogicError;
use orm_migrate::sql_models::fetcher::{
    datasource_config::{
        checkers::CheckError
    },
};
use resp_result::RespResult;
use fetcher::{
    datasource_config::OperateError as DatasourceOperateError,
    platform_config::OperateError as PlatformOperateError,
};

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
