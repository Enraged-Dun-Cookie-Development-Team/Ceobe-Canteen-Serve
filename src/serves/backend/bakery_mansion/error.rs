use axum::extract::rejection::{
    JsonRejection, PathRejection, QueryRejection,
};
use bakery_logic::error::LogicError;
use persistence::bakery::{
    mansion::OperateError, models::mansion::preludes::CheckError,
};

use crate::{error_generate};

error_generate!(
    pub MansionError
    
    Path = PathRejection
    Json = JsonRejection
    // request entity error
    Query = QueryRejection
    //db error
    Mongo = OperateError
    Check = CheckError
    Logic = LogicError
);
