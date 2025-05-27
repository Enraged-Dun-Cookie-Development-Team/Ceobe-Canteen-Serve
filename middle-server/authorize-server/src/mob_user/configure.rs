use std::{borrow::Cow, str::FromStr, sync::OnceLock};

use http::{request::Parts, HeaderName};
use tracing::warn;
use tracing_unwrap::ResultExt;

pub trait MobUserAuthConfig {
    fn header(&self) -> &str { "mob-id" }
}

static LOCAL_CONFIG: OnceLock<LocalMobUserAuthConfig> = OnceLock::new();

pub(crate) struct LocalMobUserAuthConfig {
    header: HeaderName,
}

impl LocalMobUserAuthConfig {
    pub(crate) fn from_config<C: MobUserAuthConfig>(cfg: &C) -> Self {
        let header = HeaderName::from_str(cfg.header())
            .expect_or_log("非法的请求头名称");
        Self { header }
    }
}

impl Default for LocalMobUserAuthConfig {
    fn default() -> Self {
        Self {
            header: HeaderName::from_static("mob-id"),
        }
    }
}

impl LocalMobUserAuthConfig {
    pub(crate) fn set<C: MobUserAuthConfig>(cfg: &C) {
        LOCAL_CONFIG
            .set(Self::from_config(cfg))
            .ok()
            .expect("MobUserAuth配置信息重复提供")
    }

    pub(super) fn get() -> &'static Self {
        LOCAL_CONFIG.get_or_init(|| {
            warn!(mobUserAuth.config.set = false, auth.config = "Default");
            Default::default()
        })
    }

    pub(super) fn get_header_name() -> &'static HeaderName {
        &Self::get().header
    }
}

pub(super) fn get_authorize_information(req: &Parts) -> Option<Cow<str>> {
    req.headers
        .get(LocalMobUserAuthConfig::get_header_name())
        .and_then(|v| v.to_str().ok())
        .and_then(|s| urlencoding::decode(s).ok())
}
