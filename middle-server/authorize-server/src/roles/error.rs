use status_err::{status_error, ErrPrefix};

use crate::roles::UserRoleVerify;

#[derive(Debug, thiserror::Error)]
#[error("权限不足！: 要求用户权限等级为 {0} 当前用户等级无法满足")]
pub struct AuthorizationAccessDenyError(&'static str);

impl AuthorizationAccessDenyError {
    pub fn new<T: UserRoleVerify>() -> Self { Self(T::ROLE_NAME) }
}

status_error!(
    AuthorizationAccessDenyError
    [
        ErrPrefix::UNAUTHORIZED,
        2
    ]
);
