use axum_resp_result::ConfigTrait;
use axum_starter::prepare;

use crate::utils::mob_verify;

/// rresult配置
#[prepare(RResultConfig?)]
fn resp_conf<C>(
    resp_result: &C,
) -> Result<(), axum_resp_result::SetRespResultConfigureError>
where
    C: ConfigTrait,
{
    axum_resp_result::try_set_config(resp_result)
}

/// 鉴权配置
#[prepare(BackendAuthConfig)]
fn backend_user_auth_conf<C>(user_auth: &C)
where
    C: mob_verify::MobIdConfig,
{
    mob_verify::set_auth_config(user_auth)
}
