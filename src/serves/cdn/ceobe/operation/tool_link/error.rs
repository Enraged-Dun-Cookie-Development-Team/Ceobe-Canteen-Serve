use axum::extract::rejection::{JsonRejection, QueryRejection};
use resp_result::RespResult;

use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::tool_link_mongodb::OperateMongoError;
use crate::error_generate;

error_generate! {
    pub CeobeOperateToolLinkError

    Json = JsonRejection
    Query = QueryRejection
    Logic = LogicError
    DbOperate = OperateMongoError
}

pub(super) type CeobeToolLinkRResult<T> = RespResult<T, CeobeOperateToolLinkError>;