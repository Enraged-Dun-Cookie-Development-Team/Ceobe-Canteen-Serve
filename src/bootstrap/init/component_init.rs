use axum_starter::prepare;
use resp_result::ConfigTrait;

use crate::utils::{
    mob_verify,
    user_authorize::{self, config::AuthConfig},
};

/// rresult配置
#[prepare(RResultConfig? 'arg)]
fn resp_conf<C>(
    resp_result: &'arg C,
) -> Result<(), resp_result::SetRespResultConfigureError>
where
    C: ConfigTrait,
{
    resp_result::try_set_config(resp_result)
}

/// 鉴权配置
#[prepare(BackendAuthConfig 'arg)]
fn backend_user_auth_conf<C>(user_auth: &'arg C)
where
    C: AuthConfig + mob_verify::MobIdConfig,
{
    user_authorize::set_auth_config(user_auth);
}
