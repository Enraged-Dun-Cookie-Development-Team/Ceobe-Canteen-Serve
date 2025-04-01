use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum_resp_result::RespResult;
use ceobe_operation_logic::error::LogicError;
use persistence::ceobe_operate::video;

use crate::{error_generate};

error_generate! {
    pub CeobeOperationVideoError
    
    Url = url::ParseError
    Json = JsonRejection
    Query = QueryRejection
    Check = video::CheckError
    Logic = LogicError
}

pub(super) type VideoRespResult<T> = RespResult<T, CeobeOperationVideoError>;
