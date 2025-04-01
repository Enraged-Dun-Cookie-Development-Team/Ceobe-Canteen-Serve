use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use crypto_str::inner_encoders::bcrypt::BcryptError;

crate::error_generate!(
    pub AdminUserError
    Json = JsonRejection
    Bcrypt = BcryptError
    Query = QueryRejection
    OrmDB = persistence::admin::user::OperateError
    Check = persistence::admin::models::CheckError
    PageSize = NonZeroUnsignedError
);
