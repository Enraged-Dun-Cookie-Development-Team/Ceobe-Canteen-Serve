use axum::extract::rejection::{
    JsonRejection, PathRejection, QueryRejection,
};
use persistence::bakery::mansion::OperateError;
use persistence::bakery::models::mansion::preludes::CheckError;
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
