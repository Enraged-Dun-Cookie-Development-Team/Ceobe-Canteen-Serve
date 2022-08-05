use http::StatusCode;
use orm_migrate::sql_models::admin_user::operate::OperateError;
use status_err::ErrPrefix;

use super::auth_level;
use crate::error_generate;

status_err::status_error!(
    pub TokenNotFound [
        ErrPrefix::UNAUTHORIZED,
        1
    ]=>"缺少Token字段"
);

status_err::status_error!(
    pub TokenInfoNotFound [
        ErrPrefix::UNAUTHORIZED,
        3: StatusCode::NOT_FOUND
    ]=>"Token对应信息不存在"
);

status_err::status_error!(
    pub TokenInvalid [
        ErrPrefix::UNAUTHORIZED,
        6
    ]=>"Token失效"
);
use crypto_str::inner_encoders::bcrypt::BcryptError;

error_generate!(
    pub AuthError

    Jwt = jwt::Error
    Bcrypt = BcryptError

    UserDbOperate = OperateError

    AuthLevel = auth_level::UnacceptableAuthorizationLevelError
    NoToken = TokenNotFound
    TokenInvalid = TokenInvalid
    TokenInfoNotFound = TokenInfoNotFound
);
