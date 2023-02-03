//! 带有等级鉴定功能的鉴权模块

use orm_migrate::sql_models::admin_user::AuthLevel;
pub mod prefabs;
#[macro_use]
pub mod codegen;
pub mod error;

/// 权限等级判别特征
/// 用于判决是否允许这种类型权限通过
pub trait AuthLevelVerify {
    /// 当前权限等级名称
    fn auth_name() -> &'static str;
    /// 进行权限等级鉴定。通过就返回 true 否则 返回 false;
    fn verify(token_auth: &AuthLevel) -> bool;
}
