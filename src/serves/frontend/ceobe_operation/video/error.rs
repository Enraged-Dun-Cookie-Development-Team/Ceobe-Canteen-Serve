use axum::extract::rejection::QueryRejection;
use orm_migrate::sql_models::ceobe_operation::video;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeOperationVideoError

    Query = QueryRejection
    DbOperate = video::operate::OperateError
}

pub(super) type VideoRespResult<T> = RespResult<T, CeobeOperationVideoError>;
