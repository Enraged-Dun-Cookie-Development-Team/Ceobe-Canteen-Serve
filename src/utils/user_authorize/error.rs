use crypto_str::inner_encoders::bcrypt::BcryptError;
use orm_migrate::sql_models::admin_user::OperateError;

#[derive(Debug, status_err::ThisError, status_err::StatusErr)]
pub enum AuthError {
    #[error(transparent)]
    Jwt(#[from] jwt::Error),

    #[error(transparent)]
    Bcrypt(#[from] BcryptError),

    #[error(transparent)]
    UserDbOperate(#[from] OperateError),
}
