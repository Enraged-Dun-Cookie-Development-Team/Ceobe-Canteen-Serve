use super::AuthLevelVerify;
use crate::models::common::sql::sql_models::auth_level::AuthLevel;

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

pub struct Any;

impl AuthLevelVerify for Any {
    fn auth_name() -> &'static str { "Any" }

    fn verify(_: &AuthLevel) -> bool { true }
}

pub struct Nil;

impl AuthLevelVerify for Nil {
    fn auth_name() -> &'static str { "Nil" }

    fn verify(_: &AuthLevel) -> bool { false }
}
