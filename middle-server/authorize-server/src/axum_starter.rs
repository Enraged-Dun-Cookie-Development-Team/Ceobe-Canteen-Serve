use axum_starter::prepare;
use crate::admin::{AuthConfig, LocalAuthConfig};

#[prepare(AuthorizePrepare)]
pub fn authorize_config<C>(cfg: &C)
where
    C: AuthConfig,
{
    LocalAuthConfig::set(cfg)
}
