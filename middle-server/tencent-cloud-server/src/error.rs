/// 腾讯云异常
#[derive(Debug, thiserror::Error)]
pub enum TcCloudError {
    /// 发起请求时异常
    #[error("发起推送请求异常: {0}")]
    Request(#[from] general_request_client::Error),
    /// json 序列化、反序列化异常
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Query 序列化、反序列化异常
    #[error("Query 序列化、反序列化异常: {0}")]
    Query(#[from] serde_qs::Error),
    /// HMAC加密长度错误
    #[error("HMAC加密长度错误: {0}")]
    HMACLength(#[from] hmac::digest::InvalidLength),
}
