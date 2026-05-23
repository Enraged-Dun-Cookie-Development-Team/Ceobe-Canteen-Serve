use general_request_client::http::header::InvalidHeaderValue;
use status_err::{
    generated_error::tc_cloud_kind::TcCloudError as GenTcCloudError,
    StatusErr,
};

/// 腾讯云异常
#[derive(Debug, thiserror::Error, StatusErr)]
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
    /// 腾讯云响应异常
    #[error("腾讯云响应异常: {code} => `{msg}`")]
    #[status_err(err(bind = "GenTcCloudError"))]
    TcCloud { code: String, msg: String },

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Fmt(#[from] core::fmt::Error),
}
