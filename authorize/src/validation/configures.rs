use std::str::FromStr;
use std::sync::OnceLock;
use hmac::digest::KeyInit;
use http::HeaderName;
use rand::random;
use crate::error::init_error::AuthorizeInitError;
use super::{Secret, SECRET_KEY_LENGTH};

static AUTHORIZE_CONFIG: OnceLock<AuthorizeCof> = OnceLock::new();

pub trait AuthorizeConfig {
    fn secret(&self) -> &[u8];
    fn header_name(&self) -> &str {
        "Token"
    }
}


pub(crate) struct AuthorizeCof {
    secret: Secret,
    header: HeaderName,
}

impl AuthorizeCof {
    fn try_from(value: impl AuthorizeConfig) -> Result<Self, AuthorizeInitError> {
        let rand_key = value.secret();
        if rand_key.len() < SECRET_KEY_LENGTH {
            return Err(AuthorizeInitError::SecretKeyTooShort(rand_key.len()));
        }

        let secret = Secret::new_from_slice(rand_key)?;
        let header = HeaderName::from_str(value.header_name())?;
        Ok(Self { secret, header })
    }
}

impl Default for AuthorizeCof {
    fn default() -> Self {
        let rand_key: [u8; SECRET_KEY_LENGTH] = random();
        Self {
            secret: Secret::new_from_slice(&rand_key).expect("解析JWT密钥失败"),
            header: HeaderName::from_static("Token"),
        }
    }
}

