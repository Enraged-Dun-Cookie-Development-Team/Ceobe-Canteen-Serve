use mime::Mime;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use general_request_client::Method;
use chrono::Utc;
#[derive(Debug, Clone, TypedBuilder)]
pub struct CommonParameter {
    pub service: &'static str,
    pub version: &'static str,
    pub action: &'static str,
    #[builder(default)]
    pub region: Option<String>,
    #[builder(default = "TC3-HMAC-SHA256")]
    pub algorithm: &'static str,
    #[builder(default = Utc::now().timestamp())]
    pub timestamp: i64,
    #[builder(default = "content-type;host;x-tc-action")]
    pub signed_headers: &'static str,
    #[builder(default)]
    pub token: Option<String>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct RequestContent<P, Q>
    where
        P: Serialize,
        Q: Serialize + Clone,
{
    #[builder(default = Method::POST)]
    pub method: Method,
    pub payload: P,
    #[builder(default = Option::<Q>::None, setter(strip_option))]
    pub query: Option<Q>,
    pub content_type: Mime,
}

#[derive(Debug, Clone, TypedBuilder, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TencentCloudResponse {
    pub response: ResponsePayload,
}

#[derive(Debug, Clone, TypedBuilder, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponsePayload {
    #[serde(default)]
    pub error: Option<TencentCloudError>,
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, TypedBuilder, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TencentCloudError {
    pub code: String,
    pub message: String,
}