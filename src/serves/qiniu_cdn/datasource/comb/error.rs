use axum::extract::rejection::{PathRejection, QueryRejection};
use ceobe_cookie_logic::error::LogicError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub QiniuCdnDatasourceCombError

    Logic = LogicError
    Path = PathRejection
}

pub type QiniuCdnDatasourceRResult<T> = RespResult<T, QiniuCdnDatasourceCombError>;
