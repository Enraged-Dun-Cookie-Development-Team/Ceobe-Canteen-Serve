use axum::extract::rejection::QueryRejection;
use bakery::mansion::OperateError;
use bakery_logic::error::LogicError;
use mongo_migration::mongo_models::bakery::mansion::checkers::CheckError;

use crate::error_generate;

error_generate!(
    pub MansionError
    // request entity error
    Query = QueryRejection
    //db error
    Mongo = OperateError
    Checker = CheckError
    ModifyVerify = modify_cache::Error
    BakeryLogic = LogicError
);
