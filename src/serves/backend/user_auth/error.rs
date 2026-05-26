use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use crypto_str::inner_encoders::bcrypt::BcryptError;
use status_err::{
    generated_error::checker_kind::SelfDeleteError as GenSelfDeleteError,
    status_error,
};
crate::error_generate!(
    pub AdminUserError
    Json = JsonRejection
    Bcrypt = BcryptError
    Query = QueryRejection
    OrmDB = persistence::admin::user::OperateError
    Check = persistence::admin::models::CheckError
    PageSize = NonZeroUnsignedError
    SelfDelete = SelfDeleteError
);
status_error!(
    new pub SelfDeleteError["正在试图抹除自身存在"]
    => GenSelfDeleteError
);
