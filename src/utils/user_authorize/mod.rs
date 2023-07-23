use hmac::Hmac;
use orm_migrate::sql_models::admin_user;
pub use orm_migrate::sql_models::admin_user::AuthLevel;
pub use set_token::GenerateToken;
use sha2::Sha256;
pub use valid_token::decrypt_token;

use super::mob_verify;

pub mod config;
pub mod error;

mod auth_level_check;
mod set_token;
mod valid_token;

crate::quick_struct! {

    #[derive(PartialEq, Eq)]
    pub User{
        id:i32
        num_pwd_change:u32
    }


    pub VerifiedAuthInfo{
        id:i32
        username:String
        pwd:String
    }
}

/// 用户权限信息
pub type AuthInfo = admin_user::Model;

pub fn set_auth_config<C>(cfg: &C)
where
    C: config::AuthConfig + mob_verify::MobIdConfig,
{
    config::set_auth_config(cfg);
    mob_verify::set_auth_config(cfg)
}

/// 获取jwt密钥
fn get_key() -> &'static Hmac<Sha256> { config::get_jwt_key() }

pub type PasswordEncoder =
    crypto_str::inner_encoders::bcrypt::DefaultBcryptEncoder;

/// 权限等级鉴定模块
pub mod auth_level {
    pub use super::auth_level_check::{
        error::UnacceptableAuthorizationLevelError, AuthLevelVerify,
    };

    pub mod prefabs {
        pub use super::super::auth_level_check::prefabs::*;
    }
}

#[cfg(test)]
mod test {
    use super::{set_token::GenerateToken, valid_token::decrypt_token, User};

    #[test]
    fn generate_key() {
        let user = User {
            id: 5,
            num_pwd_change: 3,
        };

        let token = user.clone().generate().unwrap();
        let valid_user = decrypt_token(&token).unwrap();

        assert_eq!(user, valid_user);
    }
}
