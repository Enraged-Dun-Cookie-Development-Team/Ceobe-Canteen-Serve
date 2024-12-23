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
use status_err::{ErrPrefix, StatusErr};

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

impl StatusErr for FieldNotExist {
    fn prefix(&self) -> status_err::ErrPrefix { ErrPrefix::CHECKER }

    fn code(&self) -> u16 { 0x0011 }
}
