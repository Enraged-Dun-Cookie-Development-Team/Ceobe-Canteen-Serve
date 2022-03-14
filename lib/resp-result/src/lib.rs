#![feature(try_trait_v2)]
mod config;
mod owner_leak;
mod resp_error;
mod resp_result;

use config::InnerConfig;
pub use config::{ConfigTrait, DefaultConfig, RespConfig, SerdeConfig};
pub use resp_error::RespError;
pub use resp_result::RespResult;

static RESP_RESULT_CONFIG: state::Storage<InnerConfig> = state::Storage::new();

pub fn set_config<C: ConfigTrait>(cfg: &C) {
    let inner = InnerConfig::from_cfg(cfg);

    let rsp = RESP_RESULT_CONFIG.set(inner);
    if !rsp {
        panic!("Resp Result 配置已经被设置了")
    }
}

pub(crate) fn get_config() -> &'static InnerConfig {
    if let Some(cfg) = RESP_RESULT_CONFIG.try_get() {
        cfg
    } else {
        #[cfg(feature = "log")]
        logger::warn!("未配置RespResult 配置文件，将使用默认配置");
        let cfg = RESP_RESULT_CONFIG.get_or_set(Default::default);
        cfg
    }
}

pub fn leak_string(s: String) -> &'static str {
    let ls = Box::leak(s.into_boxed_str()) as &'static str;
    ls
}
