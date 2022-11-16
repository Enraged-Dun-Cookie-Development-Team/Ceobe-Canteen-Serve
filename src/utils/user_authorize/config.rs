use std::borrow::Cow;

use axum::body::Body;
use hmac::{digest::KeyInit, Hmac};
use http::Request;
use once_cell::sync::OnceCell;
use sha2::Sha256;

crate::quick_trait! {
    pub AuthConfig{
        crate::trait_field!{*jwt_key:&[u8]}
        crate::trait_field!{*token_header:String=String::from("Token")}
    }
}
use tracing::log;

static LOCAL_CONFIG: OnceCell<LocalAuthConfig> = OnceCell::new();

struct LocalAuthConfig {
    jwt_key: Hmac<Sha256>,
    header: &'static str,
}

impl Default for LocalAuthConfig {
    fn default() -> Self {
        let rand_key: [u8; 32] = rand::random();
        Self {
            jwt_key: Hmac::new_from_slice(&rand_key)
                .expect("无法解析JWT KEY"),
            header: "Token",
        }
    }
}

impl LocalAuthConfig {
    pub(super) fn from_config<C: AuthConfig>(cfg: &C) -> Self {
        // generate jwt
        let jwt_key =
            Hmac::new_from_slice(cfg.jwt_key()).expect("无法解析JWT KEY");
        // generate static str
        let name = cfg.token_header().into_boxed_str();
        let header = Box::leak(name) as &'static str;

        Self { jwt_key, header }
    }
}

pub(super) fn set_auth_config<C: AuthConfig>(cfg: &C) {
    if LOCAL_CONFIG.set(LocalAuthConfig::from_config(cfg)).is_err() {
        panic!("UserAuth配置信息重复提供")
    }
}

fn get_local_config() -> &'static LocalAuthConfig {
    LOCAL_CONFIG.get_or_init(|| {
        log::warn!("Auth模块配置文件未配置，将使用默认配置信息");
        LocalAuthConfig::default()
    })
}

pub(super) fn get_jwt_key() -> &'static Hmac<Sha256> {
    let config = get_local_config();
    &config.jwt_key
}

pub fn get_header_name() -> &'static str {
    let config = get_local_config();
    config.header
}

pub fn get_authorize_information(req: &Request<Body>) -> Option<Cow<str>> {
    req.headers()
        .get(get_header_name())
        .and_then(|v| v.to_str().ok())
        .and_then(|s| urlencoding::decode(s).ok())
}
