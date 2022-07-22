use http::StatusCode;
use orm_migrate::sql_models::user::UserError;
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

error_generate!(
    pub AuthError

    Jwt = jwt::Error
    Bcrypt = bcrypt::BcryptError

    UserDbOperate = UserError

    AuthLevel = auth_level::UnacceptableAuthorizationLevelError
    NoToken = TokenNotFound
    TokenInvalid = TokenInvalid
    TokenInfoNotFound = TokenInfoNotFound
);
