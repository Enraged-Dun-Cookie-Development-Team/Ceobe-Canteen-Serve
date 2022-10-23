use axum_starter::{prepare, PreparedEffect};
use resp_result::ConfigTrait;

use crate::utils::user_authorize::{self, config::AuthConfig};

/// 请求rresult配置
#[prepare(RResultConfig 'arg)]
fn resp_conf<C>(resp_result: &'arg C) -> impl PreparedEffect
where
    C: ConfigTrait,
{
    resp_result::set_config(resp_result);
}

/// 鉴权配置
#[prepare(BackendAuthConfig 'arg)]
fn backend_user_auth_conf<C>(user_auth: &'arg C) -> impl PreparedEffect
where
    C: AuthConfig,
{
    user_authorize::set_auth_config(user_auth);
}
