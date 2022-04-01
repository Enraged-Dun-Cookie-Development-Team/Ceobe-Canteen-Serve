use http::StatusCode;
use status_err::ErrPrefix;

use super::auth_level;
use crate::error_generate;

status_err::status_error!(
    pub TokenNotFound [
        ErrPrefix::UNAUTHORIZED,
        0001
    ]=>"缺少Token字段"
);

status_err::status_error!(
    pub PasswordWrong [
        ErrPrefix::UNAUTHORIZED,
        0004
    ]=>"密码错误"
);

status_err::status_error!(
    pub UserNotFound [
        ErrPrefix::UNAUTHORIZED,
        0003:StatusCode::NOT_FOUND
    ]=>"Token对应信息不存在"
);

error_generate!(
    pub AuthError

    Jwt=jwt::Error
    NoToken = TokenNotFound
    NoUser = UserNotFound
    Password = PasswordWrong
    Actix = actix_web::Error
    Db = sea_orm::DbErr
    Bcrypto = bcrypt::BcryptError
    AuthLevel = auth_level::UnacceptableAuthorizationLevelError
);
