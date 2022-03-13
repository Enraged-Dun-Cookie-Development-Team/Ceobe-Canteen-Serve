/// 生成 Resp 时的配置
pub trait RespConfig {
    #[cfg(feature = "extra-code")]
    fn head_extra_code(&self)->Option<& 'static str>{
        Some("extra-code")
    }

}