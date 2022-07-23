use actix_web::error::QueryPayloadError;
use mongo_migration::mongo_models::mansion_data::{
    checkers::MansionDataCheckerError, MansionDataError,
};

use crate::{
    error_generate,
    utils::{
        req_pretreatment::prefabs::{JsonError, PathError},
        user_authorize::error::AuthError,
    },
};

error_generate!(
    pub MansionError
    // request entity error
    Auth = AuthError
    Path = PathError
    Json = JsonError
    Query = QueryPayloadError
    //db error
    Mongo = MansionDataError
    Check = MansionDataCheckerError
);
