mod auth_pretreator;
mod set_token;
mod valid_token;

pub use auth_pretreator::{
    AuthError, AuthInfo, AuthLevel, PasswordWrong, TokenAuth, TokenNotFound, UserNotFound,
};
pub use set_token::GenerateToken;

use super::req_pretreatment::{prefabs::MapErr, ReqPretreatment};
use hmac::{digest::InvalidLength, Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub type Authentication<E> =
    ReqPretreatment<crate::utils::req_pretreatment::prefabs::ToRResult<MapErr<TokenAuth, E>>>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct User {
    id: i32,
    password: String,
}

static JWT_KEY: state::Storage<Hmac<Sha256>> = state::Storage::new();

pub fn set_key(key: &[u8]) -> Result<(), InvalidLength> {
    if JWT_KEY.set(Hmac::new_from_slice(key)?) {
        Ok(())
    } else {
        panic!("jwt密钥重复生成")
    }
}

/// 获取jwt密钥
fn get_key() -> &'static Hmac<Sha256> {
    if let None = JWT_KEY.try_get() {
        let rand_key: [u8; 32] = rand::random();
        JWT_KEY.set(Hmac::new_from_slice(&rand_key).expect("jwt密钥生成失败"));
    }
    JWT_KEY.get()
}

pub type PasswordEncoder = crypto_str::inner_encoders::bcrypt::DefaultBcryptEncoder;

#[cfg(test)]
mod test {

    use super::{set_token::GenerateToken, valid_token::decrpyt_token, User};

    #[test]
    fn generate_key() {
        let user = User {
            id: 5,
            password: "1234asdf!@".into(),
        };

        let token = user.generate().unwrap();
        let valid_user = decrpyt_token(token).unwrap();

        let user = User {
            id: 5,
            password: "1234asdf!@".into(),
        };

        assert_eq!(user, valid_user);
    }
}
