use axum::extract::{
    multipart::{MultipartError, MultipartRejection},
    rejection::{JsonRejection, QueryRejection},
};
use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;
use ceobe_qiniu_upload::Error as QiniuError;
use checker::{
    prefabs::num_check::NonZeroUnsignedError, JsonCheckExtract,
    QueryCheckExtract,
};
use page_size::request::PageSizeChecker;
use persistence::ceobe_operate::{
    models::tool_link::{
        self, checkers::tool_link_data::PreCheckCeobeOperationToolLinkChecker,
    },
    tool_link_mongodb::OperateError,
};
use status_err::{
    generated_error::checker_kind::FieldNotExistError, status_error,
};

use crate::error_generate;

error_generate! {
    pub OperateToolLinkError

    Json = JsonRejection
    Query = QueryRejection
    LogicError = LogicError
    CheckerError = tool_link::CheckError
    PageSize = NonZeroUnsignedError
    Upload = QiniuError
    Multipart = MultipartError
    MultipartReject = MultipartRejection
    Field = FieldNotExist
    DbOperate = OperateError
}

pub type OperateToolLinkRResult<T> = RespResult<T, OperateToolLinkError>;

pub type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, OperateToolLinkError>;

pub type ToolLinkPretreatment = JsonCheckExtract<
    PreCheckCeobeOperationToolLinkChecker,
    OperateToolLinkError,
>;

#[derive(Debug, thiserror::Error)]
#[error("Field 不存在")]
pub struct FieldNotExist;

status_error!(
    FieldNotExist
    => FieldNotExistError
);
