use std::{borrow::Cow, str::FromStr, sync::OnceLock};

use axum::http::HeaderName;
use http::request::Parts;
use jsonwebtoken::{DecodingKey, EncodingKey};
use tracing::warn;
use tracing_unwrap::ResultExt;

pub trait AuthConfig {
    fn jwt_key(&self) -> &[u8];
    fn token_header(&self) -> &str { "Token" }
}
static LOCAL_CONFIG: OnceLock<LocalAuthConfig> = OnceLock::new();

pub(crate) struct LocalAuthConfig {
    encoder_key: EncodingKey,
    decoder_key: DecodingKey,
    header: HeaderName,
}

impl Default for LocalAuthConfig {
    fn default() -> Self {
        let rand_key: [u8; 32] = rand::random();
        Self {
            encoder_key: EncodingKey::from_secret(&rand_key),
            decoder_key: DecodingKey::from_secret(&rand_key),
            header: HeaderName::from_static("Token"),
        }
    }
}

impl LocalAuthConfig {
    pub(super) fn from_config<C: AuthConfig>(cfg: &C) -> Self {
        // generate jwt

        let encoder_key = EncodingKey::from_secret(cfg.jwt_key());
        let decoder_key = DecodingKey::from_secret(cfg.jwt_key());
        // generate static str
        let header = cfg.token_header();
        let header =
            HeaderName::from_str(header).expect_or_log("非法的请求头名称");

        Self {
            encoder_key,
            decoder_key,
            header,
        }
    }
}

impl LocalAuthConfig {
    pub(crate) fn set<C: AuthConfig>(cfg: &C) {
        LOCAL_CONFIG
            .set(LocalAuthConfig::from_config(cfg))
            .map_err(|_| ())
            .expect_or_log("UserAuth配置信息重复提供")
    }

    fn get() -> &'static LocalAuthConfig {
        LOCAL_CONFIG.get_or_init(|| {
            warn!(auth.config.set = false, auth.config = "Default");
            LocalAuthConfig::default()
        })
    }

    pub(crate) fn encoder_key() -> &'static EncodingKey {
        let config = Self::get();
        &config.encoder_key
    }

    pub(crate) fn decoder_key() -> &'static DecodingKey {
        let config = Self::get();
        &config.decoder_key
    }

    pub(crate) fn header_name() -> &'static HeaderName {
        let config = Self::get();
        &config.header
    }
}

pub fn get_authorize_information(req: &Parts) -> Option<Cow<str>> {
    req.headers
        .get(LocalAuthConfig::header_name())
        .and_then(|v| v.to_str().ok())
        .and_then(|s| urlencoding::decode(s).ok())
}
