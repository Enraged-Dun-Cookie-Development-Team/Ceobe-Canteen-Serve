pub mod config;
pub mod token_loader;
#[macro_use]
mod auth_level_check;

mod auth_pretreator;
mod set_token;
mod valid_token;

pub use auth_pretreator::{
    AuthError, AuthLevel, PasswordWrong, TokenAuth, TokenNotFound,
    UserNotFound,
};
use hmac::{digest::InvalidLength, Hmac, Mac};
pub use set_token::GenerateToken;
use sha2::Sha256;

use super::req_pretreatment::{prefabs::MapErr, ReqPretreatment};
use crate::utils::req_pretreatment::prefabs::ToRResult;

pub type Authentication<E> = ReqPretreatment<ToRResult<MapErr<TokenAuth, E>>>;
pub type AuthenticationLevel<L, E> =
    ReqPretreatment<ToRResult<MapErr<auth_level::AuthLevel<L>, E>>>;

crate::quick_struct! {

    #[derive(PartialEq, Eq)]
    pub User{
        id:i32
        password:String
    }

    /// 用户权限信息
    pub AuthInfo{
        id: i32
        /// 权限
        auth: AuthLevel
        username: String
    }

    pub VerifiedAuthInfo{
        id:i32
        username:String
    }
}

static JWT_KEY: state::Storage<Hmac<Sha256>> = state::Storage::new();

pub fn set_auth_config<C>(cfg: &C) -> Result<(), InvalidLength>
where
    C: config::AuthConfig,
{
    if JWT_KEY.set(Hmac::new_from_slice(config::AuthConfig::jwt_key(cfg))?) {
        Ok(())
    }
    else {
        panic!("jwt密钥重复生成")
    }
}

/// 获取jwt密钥
fn get_key() -> &'static Hmac<Sha256> {
    if let None = JWT_KEY.try_get() {
        let rand_key: [u8; 32] = rand::random();
        JWT_KEY
            .set(Hmac::new_from_slice(&rand_key).expect("jwt密钥生成失败"));
    }
    JWT_KEY.get()
}

pub type PasswordEncoder =
    crypto_str::inner_encoders::bcrypt::DefaultBcryptEncoder;

/// 权限等级鉴定模块
pub mod auth_level {
    pub use super::auth_level_check::{
        error::UnacceptableAuthorizationLevelError, pretreator::AuthLevel,
        AuthLevelVerify,
    };
    pub mod prefabs {
        pub use super::super::auth_level_check::prefabs::*;
    }
}

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
