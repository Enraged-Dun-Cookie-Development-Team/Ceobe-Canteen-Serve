static SIGNED_STATUS: &str = "is-ok";
#[cfg(feature = "extra-code")]
static EXTRA_ERR_CODE: &str = "extra-code";
static ERROR_MESSAGE: &str = "error-message";
static BODY: &str = "body";

/// 序列化时的配置信息
pub trait SerdeConfig {
    fn body_name(&self)->& 'static str{
        BODY
    }

    fn err_msg_name(&self)->& 'static str{
        ERROR_MESSAGE
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
    fn signed_base_status(&self) -> Option<& 'static str> {
        Some(SIGNED_STATUS)
    }
    /// 异常码 位置标记
    ///
    /// extra-code
    ///
    /// Some() 添加异常码标记
    /// None 不添加异常码标记
    #[cfg(feature = "extra-code")]
    fn extra_code_local(&self) -> Option<& 'static str> {
        Some(EXTRA_ERR_CODE)
    }
}
