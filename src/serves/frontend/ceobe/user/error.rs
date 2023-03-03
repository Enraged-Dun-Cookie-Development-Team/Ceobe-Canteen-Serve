use axum::extract::rejection::{JsonRejection, QueryRejection};
use ceobe_user_logic::error::LogicError;
use mongo_migration::mongo_models::ceobe::user_property::check::CheckError as CeobeUserPropertyCheckerror;
use resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub CeobeUserError

    Json = JsonRejection
    Query = QueryRejection
    Logic = LogicError
    Check = CeobeUserPropertyCheckerror
}

pub type CeobeUserRResult<T> = RespResult<T, CeobeUserError>;
