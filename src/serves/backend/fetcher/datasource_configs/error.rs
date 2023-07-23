use axum::extract::{
    multipart::{MultipartError, MultipartRejection},
    rejection::{JsonRejection, QueryRejection},
};
use ceobe_qiniu_upload::Error as QiniuError;
use checker::prefabs::num_check::NonZeroUnsignedError;
use fetcher_logic::error::LogicError;
use persistence::fetcher::{
    datasource_config::OperateError as DatasourceOperateError,
    models::datasource_config::checkers::CheckError,
    platform_config::OperateError as PlatformOperateError,
};
use resp_result::RespResult;
use status_err::{ErrPrefix, StatusErr};

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

impl StatusErr for FieldNotExist {
    fn prefix(&self) -> status_err::ErrPrefix { ErrPrefix::CHECKER }

    fn code(&self) -> u16 { 0x0011 }
}

pub type DatasourceConfigRResult<T> = RespResult<T, DatasourceConfigError>;
