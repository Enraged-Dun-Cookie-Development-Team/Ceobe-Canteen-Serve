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
    OrmDB = persistence::admin::user::OperateError
    Check = persistence::admin::models::CheckError
    PageSize = NonZeroUnsignedError
);
