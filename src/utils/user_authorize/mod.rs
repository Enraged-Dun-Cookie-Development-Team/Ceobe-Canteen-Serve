pub use authorize_server::UserClaim as User;
use persistence::admin;

/// 用户权限信息
pub type AuthInfo = admin::models::Model;


/// 权限等级鉴定模块
pub mod auth_level {
    pub mod prefabs {
        pub use authorize_server::roles::base_roles::*;
    }
}
pub type PasswordEncoder =
crypto_str::inner_encoders::bcrypt::DefaultBcryptEncoder;




pub mod config{
}