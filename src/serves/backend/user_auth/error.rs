use axum::extract::rejection::{JsonRejection, QueryRejection};
use checker::prefabs::num_check::NonZeroUnsignedError;
use crypto_str::inner_encoders::bcrypt::BcryptError;
use status_err::status_error;

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
use status_err::ErrPrefix;
status_error!(pub SelfDeleteError[ErrPrefix::CHECKER,0018]=>"正在试图抹除自身存在");