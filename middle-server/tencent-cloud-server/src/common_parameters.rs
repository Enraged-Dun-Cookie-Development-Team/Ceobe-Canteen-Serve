use std::fmt::format;

use chrono::{DateTime, Utc};
use general_request_client::Method;
use hmac::{digest::InvalidLength, Hmac, Mac};
use secrecy::ExposeSecret;
use serde::Serialize;
use sha2::{Digest, Sha256};
use typed_builder::TypedBuilder;

use crate::{
    cloud_manager::CloudManager, config::TencentConfigTrait,
    error::TcCloudError,
};

#[derive(Debug, Clone, TypedBuilder)]
pub struct CommonParameter {
    pub service: String,
    pub version: String,
    pub action: String,
    pub region: Option<String>,
    #[builder(default = String::from("TC3-HMAC-SHA256"))]
    pub algorithm: String,
    #[builder(default = Utc::now().timestamp())]
    pub timestamp: i64,
    #[builder(default = String::from("content-type;host;x-tc-action"))]
    pub signed_headers: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct RequestContent<T: Serialize, P: Serialize> {
    pub method: Method,
    pub payload: T,
    pub param: P,
    pub content_type: String,
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

impl CloudManager {
    /// 腾讯云签名函数，签名参考：https://cloud.tencent.com/document/api/228/30978
    fn sign<T: Serialize, P: Serialize>(
        &self, common_params: CommonParameter, request: RequestContent<T, P>,
    ) -> Result<String, TcCloudError> {
        let algorithm = String::from("TC3-HMAC-SHA256");
        // URI 参数，API 3.0 固定为正斜杠（/）。
        let canonical_uri = String::from("/");
        let canonical_query = serde_qs::to_string(&request.param)?;
        let host = format!("{}.tencentcloudapi.com", common_params.service);
        let canonical_headers = format!(
            "content-type:{}\nhost:{}\nx-tc-action:{}\n",
            request.content_type,
            host,
            common_params.action.to_lowercase()
        );
        // 与canonical_headers对应，目前只看到用这三个字段
        let signed_headers = String::from("content-type;host;x-tc-action");

        let payload_text = serde_json::to_string(&request.payload)?;
        let hashed_request_payload = sha256hex(&payload_text);

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            request.method.to_string(),
            canonical_uri,
            canonical_query,
            canonical_headers,
            signed_headers,
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
            algorithm,
            common_params.timestamp,
            credential_scope,
            hashed_credential_request
        );

        let secret_date =
            hmacsha256(&date, &format!("TC3{}", self.id.expose_secret()))?;
        let secret_service =
            hmacsha256(&common_params.service, &secret_date)?;
        let secret_signing = hmacsha256("tc3_request", &secret_service)?;
        let signature =
            hex::encode(hmacsha256(&string_to_sign, &secret_signing)?);

        Ok(format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            algorithm,
            self.id.expose_secret(),
            credential_scope,
            signed_headers,
            signature
        ))
    }
}
