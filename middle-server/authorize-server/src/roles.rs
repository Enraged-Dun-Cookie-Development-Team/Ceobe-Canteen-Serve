pub mod base_roles;
mod mix_role_gen;
mod error;

use persistence::admin::models::AuthLevel;

/// 用户角色权限判定
/// 判定用户是否有权限访问
pub trait UserRoleVerify{
    
    /// 特定权限等级名称
    const ROLE_NAME:&'static str;
    
    /// 判断特定用户权限等级能否允许访问
    fn access_verify(level:&AuthLevel)->bool;
}

pub use base_roles::{Any,Architect,Nil,Chef,Cooker,Porter};
pub use error::AuthorizationAccessDenyError;
