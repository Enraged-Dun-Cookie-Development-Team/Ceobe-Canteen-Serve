use std::borrow::Cow;

use axum::body::Body;
use http::Request;
use once_cell::sync::OnceCell;

crate::quick_trait! {
    pub MobIdConfig{
        crate::trait_field!{*mob_header:String=String::from("mob-id")}
    }
}
use tracing::warn;

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
