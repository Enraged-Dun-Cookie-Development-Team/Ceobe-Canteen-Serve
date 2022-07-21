use orm_migrate::sql_models::common::CommonError;
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
    pub TokenInvalid [
        ErrPrefix::UNAUTHORIZED,
        6
    ]=>"Token失效"
);

error_generate!(
    pub AuthError

    Jwt = jwt::Error
    NoToken = TokenNotFound
    UserDbOperate = CommonError
    Bcrypt = bcrypt::BcryptError
    AuthLevel = auth_level::UnacceptableAuthorizationLevelError
    TokenInvalid = TokenInvalid
);
