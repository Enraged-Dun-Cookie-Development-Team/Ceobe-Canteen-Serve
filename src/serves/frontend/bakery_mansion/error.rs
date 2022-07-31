use axum::extract::rejection::QueryRejection;
use mongo_migration::mongo_models::bakery::mansion::{
    checkers::CheckError, operate::OperateError,
};

use crate::error_generate;

error_generate!(
    pub MansionError
    // request entity error
    Query = QueryRejection
    //db error
    Mongo = OperateError
    Checker = CheckError
);
