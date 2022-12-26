use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use orm_migrate::sql_models::fetcher::datasource_config::operate::OperateError as DatasourceOperateError;
use orm_migrate::sql_models::fetcher::platform_config::operate::OperateError as PlatformOperateError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub DatasourceConfigError

    Json = JsonRejection
     Query = QueryRejection
    DatasourceOperate = DatasourceOperateError
    PlatformOperate = PlatformOperateError
    PageSize = NonZeroUnsignedError
}

pub type DatasourceConfigRResult<T> = RespResult<T, DatasourceConfigError>;