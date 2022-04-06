pub mod config;
pub mod error;
pub mod token_loader;

mod auth_level_check;
mod auth_pretreator;
mod set_token;
mod valid_token;

pub use auth_pretreator::{AuthLevel, TokenAuth};
use hmac::Hmac;
use sea_orm::FromQueryResult;
pub use set_token::GenerateToken;
use sha2::Sha256;

use super::req_pretreatment::{prefabs::MapErr, ReqPretreatment};
use crate::{models, utils::req_pretreatment::prefabs::ToRResult};

pub type Authentication<E> = ReqPretreatment<ToRResult<MapErr<TokenAuth, E>>>;
pub type AuthenticationLevel<L, E> =
    ReqPretreatment<ToRResult<MapErr<auth_level::AuthLevel<L>, E>>>;

crate::quick_struct! {

    #[derive(PartialEq, Eq, FromQueryResult)]
    pub User{
        id:i32
        password:String
    }

    
    pub VerifiedAuthInfo{
        id:i32
        username:String
        pwd:String
    }
}

/// 用户权限信息
pub type AuthInfo = models::common::sql::user::Model;

pub fn set_auth_config<C>(cfg: &C)
where
    C: config::AuthConfig,
{
    config::set_auth_config(cfg)
}

/// 获取jwt密钥
fn get_key() -> &'static Hmac<Sha256> { config::get_jwt_key() }

pub type PasswordEncoder =
    crypto_str::inner_encoders::bcrypt::DefaultBcryptEncoder;

/// 权限等级鉴定模块
pub mod auth_level {
    pub use super::auth_level_check::{
        AuthLevelVerify, error::UnacceptableAuthorizationLevelError,
        pretreator::AuthLevel,
    };
    pub mod prefabs {
        pub use super::super::auth_level_check::prefabs::*;
    }
}

#[cfg(test)]
mod test {

    use super::{set_token::GenerateToken, User, valid_token::decrpyt_token};

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
