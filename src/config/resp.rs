#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use crate::owner_leak::OwnerLeaker;
/// 生成 Resp 时的配置
pub trait RespConfig {
    #[cfg(feature = "extra-code")]
    fn head_extra_code(&self) -> Option<Cow<'static, str>> {
        Some("extra-code".into())
    }
}

pub(crate) struct InnerRespConfig {
    #[cfg(feature = "extra-code")]
    pub(crate) extra_code: Option<&'static str>,
}

impl InnerRespConfig {
    #[allow(unused_variables)]
    pub fn into_inner<C: RespConfig>(cfg: &C) -> Self {
        Self {
            #[cfg(feature = "extra-code")]
            extra_code: cfg.head_extra_code().leak(),
        }
    }
}
