use actix_web::error::QueryPayloadError;

use crate::utils::{
    req_pretreatment::prefabs::JsonError, user_authorize::error::AuthError,
};

crate::error_generate!(
    pub AdminUserError
    Json = JsonError
    OrmDB = orm_migrate::sql_models::user::UserError
    Bcrypt = bcrypt::BcryptError
    Auth = AuthError
    Query = QueryPayloadError
    Limit = range_limit::Error
);
