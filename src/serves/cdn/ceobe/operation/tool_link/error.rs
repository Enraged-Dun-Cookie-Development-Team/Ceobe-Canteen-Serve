use axum::extract::rejection::{JsonRejection, QueryRejection};
use ceobe_operation_logic::error::LogicError;
use checker::prefabs::num_check::NonZeroUnsignedError;
use persistence::ceobe_operate::tool_link_mongodb::{
    CheckError, OperateMongoError,
};
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperateToolLinkError

    Json = JsonRejection
    Query = QueryRejection
    Logic = LogicError
    DbOperate = OperateMongoError
    Check = CheckError
    PageSize = NonZeroUnsignedError
}
pub(super) type CeobeToolLinkRResult<T> =
    RespResult<T, CeobeOperateToolLinkError>;
