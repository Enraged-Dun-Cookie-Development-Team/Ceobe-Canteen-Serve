use persistence::admin::models::AuthLevel;

use super::AuthLevelVerify;

#[derive(Clone)]
pub struct Chef;

impl AuthLevelVerify for Chef {
    fn auth_name() -> &'static str { "Chef" }

    fn verify(token_auth: &AuthLevel) -> bool {
        matches!(token_auth, &AuthLevel::Chef)
    }
}
#[derive(Clone)]
pub struct Cooker;

impl AuthLevelVerify for Cooker {
    fn auth_name() -> &'static str { "Cooker" }

    fn verify(token_auth: &AuthLevel) -> bool {
        matches!(token_auth, &AuthLevel::Cooker)
    }
}

#[derive(Clone)]
pub struct Architect;

impl AuthLevelVerify for Architect {
    fn auth_name() -> &'static str { "Architect" }

    fn verify(token_auth: &AuthLevel) -> bool {
        matches!(token_auth, &AuthLevel::Architect)
    }
}

#[derive(Clone)]
pub struct Outsourcing;

impl AuthLevelVerify for Outsourcing {
    fn auth_name() -> &'static str { "Outsourcing" }

    fn verify(token_auth: &AuthLevel) -> bool {
        matches!(token_auth, &AuthLevel::Outsourcing)
    }
}

#[derive(Clone)]
pub struct Any;

impl AuthLevelVerify for Any {
    fn auth_name() -> &'static str { "Any" }

    fn verify(_: &AuthLevel) -> bool { true }
}
#[derive(Clone)]
pub struct Nil;

impl AuthLevelVerify for Nil {
    fn auth_name() -> &'static str { "Nil" }

    fn verify(_: &AuthLevel) -> bool { false }
}
