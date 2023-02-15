use axum::extract::rejection::{JsonRejection, QueryRejection};
use ceobe_user_logic::error::LogicError;
use mongo_migration::mongo_models::ceobe::user::check::CheckError as CeobeUserCheckError;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeUserError

    Json = JsonRejection
    Query = QueryRejection
    Logic = LogicError
    Check = CeobeUserCheckError
}

pub type CeobeUserRResult<T> = RespResult<T, CeobeUserError>;
