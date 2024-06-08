use status_err::{ErrPrefix, StatusErr};

/// 腾讯云异常
#[derive(Debug, thiserror::Error, StatusErr)]
pub enum TcCloudError {
    /// 发起请求时异常
    #[error("发起推送请求异常: {0}")]
    #[status_err(err = "transparent")]
    Request(#[from] general_request_client::Error),
    /// json 序列化、反序列化异常
    #[error(transparent)]
    #[status_err(err = "transparent")]
    Json(#[from] serde_json::Error),
    /// Query 序列化、反序列化异常
    #[error("Query 序列化、反序列化异常: {0}")]
    #[status_err(err = "transparent")]
    Query(#[from] serde_qs::Error),
    /// HMAC加密长度错误
    #[error("HMAC加密长度错误: {0}")]
    #[status_err(err = "transparent")]
    HMACLength(#[from] hmac::digest::InvalidLength),
    /// 腾讯云响应异常
    #[error("腾讯云响应异常: {code} => `{msg}`")]
    #[status_err(err(
        prefix = "ErrPrefix::TC_CLOUD",
        err_code = 0x0001,
        resp_msg = "腾讯云响应异常"
    ))]
    TcCloud { code: String, msg: String },
}
