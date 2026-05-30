use axum::extract::{
    multipart::{MultipartError, MultipartRejection},
    rejection::{JsonRejection, QueryRejection},
};
use axum_resp_result::RespResult;
use ceobe_qiniu_upload::Error as QiniuError;
use checker::prefabs::num_check::NonZeroUnsignedError;
use fetcher_logic::error::LogicError;
use persistence::fetcher::{
    datasource_config::OperateError as DatasourceOperateError,
    models::datasource_config::checkers::CheckError,
    platform_config::OperateError as PlatformOperateError,
};
use status_err::{
    generated_error::checker_kind::FieldNotExistError, status_error,
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
    Upload = QiniuError
    Multipart = MultipartError
    MultipartReject = MultipartRejection
    Field = FieldNotExist
}

#[derive(Debug, thiserror::Error)]
#[error("Field 不存在")]
pub struct FieldNotExist;

status_error!(
    FieldNotExist
    => FieldNotExistError
);

pub type DatasourceConfigRResult<T> = RespResult<T, DatasourceConfigError>;
