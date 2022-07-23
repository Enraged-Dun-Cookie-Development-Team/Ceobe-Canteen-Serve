use axum::extract::rejection::QueryRejection;
use mongo_migration::mongo_models::mansion_data::{
    checkers::MansionDataCheckerError, MansionDataError,
};

use crate::error_generate;

error_generate!(
    pub MansionError
    // request entity error
    Query = QueryRejection
    //db error
    Mongo = MansionDataError
    Checker = MansionDataCheckerError
);
