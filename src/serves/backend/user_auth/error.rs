use axum::extract::rejection::{JsonRejection, QueryRejection};

use crate::utils::user_authorize::error::AuthError;

crate::error_generate!(
    pub AdminUserError
    Json = JsonRejection
    OrmDB = orm_migrate::sql_models::admin_user::operate::OperateError
    Bcrypt = bcrypt::BcryptError
    Auth = AuthError
    Query = QueryRejection
    Limit = range_limit::Error
);
