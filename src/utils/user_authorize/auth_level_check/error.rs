use status_err::ErrPrefix;

#[derive(Debug)]
pub struct UnacceptableAuthorizationLevelError {
    authorization_level_name: &'static str,
}

impl UnacceptableAuthorizationLevelError {
    pub fn new(authorization_level_name: &'static str) -> Self {
        Self {
            authorization_level_name,
        }
    }
}

impl std::fmt::Display for UnacceptableAuthorizationLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "权限不足！: 要求用户权限等级为 {} 当前用户等级无法满足",
            self.authorization_level_name
        )
    }
}

impl std::error::Error for UnacceptableAuthorizationLevelError {}

status_err::status_error!(UnacceptableAuthorizationLevelError[ErrPrefix::UNAUTHORIZED,0002]);
