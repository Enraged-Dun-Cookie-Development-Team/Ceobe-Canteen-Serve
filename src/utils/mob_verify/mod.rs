use std::borrow::Cow;

use axum::body::Body;
use http::Request;
use once_cell::sync::OnceCell;
use tracing::warn;

crate::quick_trait! {
    pub MobIdConfig{
        crate::trait_field!{*mob_header:String=String::from("mob-id")}
    }
}
static LOCAL_CONFIG: OnceCell<LocalMobIdConfig> = OnceCell::new();

struct LocalMobIdConfig {
    mob_header: &'static str,
}

impl Default for LocalMobIdConfig {
    fn default() -> Self {
        Self {
            mob_header: "mob-id",
        }
    }
}

impl LocalMobIdConfig {
    pub(super) fn from_config<C: MobIdConfig>(cfg: &C) -> Self {
        // generate static str
        let name = cfg.mob_header().into_boxed_str();
        let mob_header = Box::leak(name) as &'static str;

        Self { mob_header }
    }
}

pub fn set_auth_config<C: MobIdConfig>(cfg: &C) {
    if LOCAL_CONFIG
        .set(LocalMobIdConfig::from_config(cfg))
        .is_err()
    {
        panic!("UserAuth配置信息重复提供")
    }
}

fn get_local_config() -> &'static LocalMobIdConfig {
    LOCAL_CONFIG.get_or_init(|| {
        warn!(auth.config.set = false, auth.config = "Default");
        LocalMobIdConfig::default()
    })
}

pub fn get_header_name() -> &'static str {
    let config = get_local_config();
    config.mob_header
}

pub fn get_mob_information(req: &Request<Body>) -> Option<Cow<str>> {
    req.headers()
        .get(get_header_name())
        .and_then(|v| v.to_str().ok())
        .and_then(|s| urlencoding::decode(s).ok())
}
