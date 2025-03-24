use axum_starter::prepare;

use crate::configure::{AuthConfig, LocalAuthConfig};

#[prepare(Authorize)]
pub fn authorize_config<C>(cfg: &C)
where
    C: AuthConfig,
{
    LocalAuthConfig::set(cfg)
}
