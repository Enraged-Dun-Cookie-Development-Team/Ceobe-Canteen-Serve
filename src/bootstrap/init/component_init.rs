use axum_starter::{prepare, PreparedEffect};

use crate::{
    configs::{
        auth_config::AuthConfig, resp_result_config::RespResultConfig,
    },
    utils::user_authorize,
};

/// 请求rresult配置
#[prepare(RResultConfig 'arg)]
async fn resp_conf(
    resp_result: &'arg RespResultConfig,
) -> impl PreparedEffect {
    resp_result::set_config(resp_result);
}

/// 鉴权配置
#[prepare(BackendAuthConfig 'arg)]
async fn backend_user_auth_conf(
    user_auth: &'arg AuthConfig,
) -> impl PreparedEffect {
    user_authorize::set_auth_config(user_auth);
}
