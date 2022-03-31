use super::AuthLevelVerify;
use crate::utils::user_authorize::AuthLevel;

pub struct Chef;

impl AuthLevelVerify for Chef {
    fn auth_name() -> &'static str { "Chef" }

    fn verify(token_auth: &AuthLevel) -> bool {
        matches!(token_auth, &AuthLevel::Chef)
    }
}
pub struct Cooker;

impl AuthLevelVerify for Cooker {
    fn auth_name() -> &'static str { "Cooker" }

    fn verify(token_auth: &AuthLevel) -> bool {
        matches!(token_auth, &AuthLevel::Cooker)
    }
}

pub struct Architect;

impl AuthLevelVerify for Architect {
    fn auth_name() -> &'static str { "Architect" }

    fn verify(token_auth: &AuthLevel) -> bool {
        matches!(token_auth, &AuthLevel::Architect)
    }
}
