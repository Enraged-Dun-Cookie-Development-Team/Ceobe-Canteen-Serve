use std::borrow::Cow;

use crate::owner_leak::OwnerLeaker;

static SIGNED_STATUS: &str = "is-ok";
#[cfg(feature = "extra-code")]
static EXTRA_ERR_CODE: &str = "extra-code";
static ERROR_MESSAGE: &str = "error-message";
static BODY: &str = "body";

/// 序列化时的配置信息
pub trait SerdeConfig {
    fn body_name(&self) -> Cow<'static, str> {
        BODY.into()
    }

    fn err_msg_name(&self) -> Cow<'static, str> {
        ERROR_MESSAGE.into()
    }

    /// 无论如何，字段数目都固定, 不需要的字段使用null填充 true
    /// 只提供需要的字段,其他缺省
    fn full_field(&self) -> bool {
        true
    }

    /// 标记基本响应状态
    /// - true 正常响应
    /// - false 异常响应
    ///
    /// is-ok
    ///
    /// Some() 标记，字段为提供的名称
    /// None 不标记
    fn signed_base_status(&self) -> Option<Cow<'static, str>> {
        Some(SIGNED_STATUS.into())
    }
    /// 异常码 位置标记
    ///
    /// extra-code
    ///
    /// Some() 添加异常码标记
    /// None 不添加异常码标记
    #[cfg(feature = "extra-code")]
    fn extra_code(&self) -> Option<Cow<'static, str>> {
        Some(EXTRA_ERR_CODE.into())
    }
}

pub(crate) struct InnerSerdeConfig {
    pub(crate) body_name: &'static str,
    pub(crate) err_msg_name: &'static str,
    pub(crate) full_field: bool,
    pub(crate) signed_base_status: Option<&'static str>,
    #[cfg(feature = "extra-code")]
    pub(crate) extra_code: Option<&'static str>,
}

impl InnerSerdeConfig {
    pub(crate) fn into_inner<C: SerdeConfig>(cfg: &C) -> Self {
        Self {
            body_name: cfg.body_name().leak(),
            err_msg_name: cfg.err_msg_name().leak(),
            full_field: cfg.full_field(),
            signed_base_status: cfg.signed_base_status().leak(),
            #[cfg(feature = "extra-code")]
            extra_code: cfg.extra_code().leak(),
        }
    }
}
