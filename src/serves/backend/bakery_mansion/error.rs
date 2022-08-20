use axum::extract::rejection::{
    JsonRejection, PathRejection, QueryRejection,
};
use mongo_migration::mongo_models::bakery::mansion::{
    checkers::CheckError, operate::OperateError,
};

use crate::{error_generate, utils::user_authorize::error::AuthError};

error_generate!(
    pub MansionError
    // request entity error
    Auth = AuthError
    Path = PathRejection
    Json = JsonRejection
    Query = QueryRejection
    //db error
    Mongo = OperateError
    Check = CheckError
);
