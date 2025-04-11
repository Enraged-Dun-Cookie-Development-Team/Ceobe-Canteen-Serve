use axum_resp_result::ConfigTrait;
use axum_starter::prepare;


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
