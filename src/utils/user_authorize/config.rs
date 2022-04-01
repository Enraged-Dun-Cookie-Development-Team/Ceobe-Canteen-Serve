use std::collections::HashMap;
use hmac::digest::KeyInit;
use hmac::Hmac;
use sha2::Sha256;
use state::Storage;


crate::quick_trait! {
    pub AuthConfig{
        crate::trait_field!{*jwt_key:&[u8]}
        crate::trait_field!{*token_header:String=String::from("Token")}
    }
}

pub(super) static LOCAL_CONFIG:Storage<LocalAuthConfig>=Storage::new();

pub(super) struct LocalAuthConfig {
    jwt_key: Hmac<Sha256>,
    header: &'static str,
}

impl Default for LocalAuthConfig {
    fn default() -> Self {
        let rand_key:[u8;32]=rand::random();
        Self{
            jwt_key:Hmac::new_from_slice(&rand_key).expect("无法解析JWT KEY"),
            header:"Token"
        }
    }
}

impl LocalAuthConfig {
    pub(super) fn from_config<C: AuthConfig>(cfg: &C) -> Self {
        // generate jwt
        let jwt_key = Hmac::new_from_slice(cfg.jwt_key())
            .expect("无法解析JWT KEY");
        // generate static str
        let name = cfg.token_header().into_boxed_str();
        let header = Box::leak(name) as &'static str;

        Self { jwt_key, header }
    }
}

pub(super) fn get_local_config() -> &'static LocalAuthConfig {
    if let Some(config)=LOCAL_CONFIG.try_get(){
        config
    }else{
        log::warn!("Auth模块配置文件未配置，将使用默认配置信息");
        LOCAL_CONFIG.get_or_set(LocalAuthConfig::default)
    }
}

pub(super) fn get_jwt_key() -> &'static Hmac<Sha256> {
    let config = get_local_config();
    &config.jwt_key
}

fn get_header_name() -> &'static str {
    let config = get_local_config();
    config.header
}

pub(super) struct TokenHeader;

impl crate::utils::data_struct::header_info::FromHeaders for TokenHeader {
    fn header_name() -> &'static str {
        get_header_name()
    }
}