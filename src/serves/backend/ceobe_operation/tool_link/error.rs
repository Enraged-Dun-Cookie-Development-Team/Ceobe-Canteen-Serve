use axum::extract::rejection::{JsonRejection, QueryRejection};
use ceobe_operation_logic::error::LogicError;
use ceobe_operation_logic::impletements::CeobeOperateLogic;
use checker::QueryCheckExtract;
use checker::prefabs::num_check::NonZeroUnsignedError;
use page_size::request::PageSizeChecker;
use persistence::ceobe_operate::models::tool_link;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub OperateToolLinkError
    
    Json = JsonRejection
    Query = QueryRejection
    LogicError = LogicError
    CheckerError = tool_link::CheckError
    PageSize = NonZeroUnsignedError
}

pub type OperateToolLinkRResult<T> = RespResult<T, OperateToolLinkError>;

pub type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, OperateToolLinkError>;