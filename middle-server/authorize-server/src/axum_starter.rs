use axum_starter::prepare;

use crate::{
    admin::{AuthConfig, LocalAuthConfig},
    mob_user::{LocalMobUserAuthConfig, MobUserAuthConfig},
};

#[prepare(AdminAuthorizePrepare)]
pub fn admin_authorize_config<C>(cfg: &C)
where
    C: AuthConfig,
{
    LocalAuthConfig::set(cfg)
}

#[prepare(MobUserAuthorizePrepare)]
pub fn mob_user_authorize_config<C>(cfg: &C)
where
    C: MobUserAuthConfig,
{
    LocalMobUserAuthConfig::set(cfg)
}
