use chrono::{DateTime, Utc};
use general_request_client::{HeaderValue, Method};
use hmac::{digest::InvalidLength, Hmac, Mac};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use typed_builder::TypedBuilder;

use crate::{
    cloud_manager::TcCloudManager, error::TcCloudError,
    requester::TencentCloudRequester,
};

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
pub struct RequestContent<P: Serialize, Q: Serialize + Clone> {
    #[builder(default = Method::POST)]
    pub method: Method,
    pub payload: P,
    pub query: Q,
    pub content_type: String,
}

#[derive(Debug, Clone, TypedBuilder, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TcCloudResponse {
    pub response: ResponseInfo,
}

#[derive(Debug, Clone, TypedBuilder, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseInfo {
    #[serde(default)]
    pub error: Option<ErrorInfo>,
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, TypedBuilder, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
}

fn sha256hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn hmacsha256(s: &str, key: &str) -> Result<String, InvalidLength> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())?;
    mac.update(s.as_bytes());
    let result = mac.finalize().into_bytes();
    Ok(hex::encode(result))
}

impl TcCloudManager {
    /// 腾讯云签名函数，签名参考：https://cloud.tencent.com/document/api/228/30978
    fn sign<P: Serialize, Q: Serialize + Clone>(
        &self, common_params: &CommonParameter,
        request: &RequestContent<P, Q>,
    ) -> Result<String, TcCloudError> {
        const ALGORITHM: &str = "TC3-HMAC-SHA256";
        // URI 参数，API 3.0 固定为正斜杠（/）。
        const CANONICAL_URI: &str = "/";
        let canonical_query = serde_qs::to_string(&request.query)?;
        let host = format!("{}.tencentcloudapi.com", common_params.service);
        let canonical_headers = format!(
            "content-type:{}\nhost:{}\nx-tc-action:{}\n",
            request.content_type,
            host,
            common_params.action.to_lowercase()
        );
        // 与canonical_headers对应，目前只看到用这三个字段
        const SIGNED_HEADERS: &str = "content-type;host;x-tc-action";

        let payload_text = serde_json::to_string(&request.payload)?;
        let hashed_request_payload = sha256hex(&payload_text);

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            request.method,
            CANONICAL_URI,
            canonical_query,
            canonical_headers,
            SIGNED_HEADERS,
            hashed_request_payload
        );

        let datetime =
            DateTime::from_timestamp(common_params.timestamp, 0).unwrap();
        let date = datetime.format("%Y-%m-%d").to_string();
        let credential_scope =
            format!("{}/{}/tc3_request", date, common_params.service);
        let hashed_credential_request = sha256hex(&canonical_request);
        let string_to_sign = format!(
            "{}\n{}\n{}\n{}",
            ALGORITHM,
            common_params.timestamp,
            credential_scope,
            hashed_credential_request
        );

        let secret_date =
            hmacsha256(&date, &format!("TC3{}", self.key.expose_secret()))?;
        let secret_service =
            hmacsha256(&common_params.service, &secret_date)?;
        let secret_signing = hmacsha256("tc3_request", &secret_service)?;
        let signature =
            hex::encode(hmacsha256(&string_to_sign, &secret_signing)?);

        Ok(format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            ALGORITHM,
            self.id.expose_secret(),
            credential_scope,
            SIGNED_HEADERS,
            signature
        ))
    }

    /// 通用请求
    pub(crate) async fn common_request<P: Serialize, Q: Serialize + Clone>(
        &self, common_params: &CommonParameter,
        request: &RequestContent<P, Q>,
    ) -> Result<TcCloudResponse, TcCloudError> {
        let authorization = self.sign(common_params, request)?;

        let mut payload_buffer = Vec::<u8>::new();
        serde_json::to_writer(&mut payload_buffer, &request.payload)?;

        let requester = TencentCloudRequester::builder()
            .url(format!(
                "https://{}.tencentcloudapi.com",
                common_params.service
            ))
            .method(request.method.clone())
            .query(request.query.clone())
            .payload(payload_buffer)
            .host(HeaderValue::from_str(&format!("{}.tencentcloudapi.com", common_params.service))?)
            .action(HeaderValue::from_str(&common_params.action)?)
            .version(HeaderValue::from_str(&common_params.version)?)
            .timestamp(HeaderValue::from_str(&common_params.timestamp.to_string())?)
            .content_type(HeaderValue::from_str(&request.content_type)?)
            .authorization(HeaderValue::from_str(&authorization)?)
            .region(common_params.region.clone().map(|region| {HeaderValue::from_str(&region).map_err(|err| TcCloudError::from(err)).ok()}).flatten())
            .token(common_params.token.clone().map(|token| {HeaderValue::from_str(&token).map_err(|err| TcCloudError::from(err)).ok()}).flatten())
            .build();

        let resp = self.client.send_request(requester).await?;

        let payload = resp.bytes().await?;
        println!("{}", String::from_utf8_lossy(&payload));

        let resp = serde_json::from_slice::<TcCloudResponse>(&payload)?;

        if let Some(error_info) = resp.response.error {
            return Err(TcCloudError::TcCloud {
                code: error_info.code,
                msg: error_info.message,
            });
        }

        Ok(resp)
    }
}
