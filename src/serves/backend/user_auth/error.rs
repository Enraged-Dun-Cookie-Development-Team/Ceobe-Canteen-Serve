use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use crypto_str::inner_encoders::bcrypt::BcryptError;

use crate::utils::user_authorize::error::AuthError;

crate::error_generate!(
    pub AdminUserError
    Json = JsonRejection
    Bcrypt = BcryptError
    Auth = AuthError
    Query = QueryRejection
    OrmDB = orm_migrate::sql_models::admin_user::operate::OperateError
    Check = orm_migrate::sql_models::admin_user::checkers::CheckError
    PageSize = NonZeroUnsignedError
);