use chrono::DateTime;
use hmac::{digest::InvalidLength, Hmac, Mac};
use serde::Serialize;
use sha2::{Digest, Sha256};
use url::Url;

use crate::{
    error::TcCloudError,
};
use crate::cloud_manager::entities::{CommonParameter, RequestContent};

/// 签名使用的算法
const ALGORITHM: &str = "TC3-HMAC-SHA256";
/// URI 参数，API 3.0 固定为正斜杠（/）。
const CANONICAL_URI: &str = "/";

/// 参与签名的请求头， 与canonical_headers对应，目前只看到用这三个字段
const SIGNED_HEADERS: &str = "content-type;host;x-tc-action";

/// 计算 [u8] slice 的 [sha256](Sha256) 值，并转换为16进制的 [String] 
fn sha256hex(s: &impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_ref());
    let result = hasher.finalize();
    hex::encode(result)
}

fn hmac_sha256(
    s: &impl AsRef<[u8]>, key: &impl AsRef<[u8]>,
) -> Result<Vec<u8>, InvalidLength> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key.as_ref())?;
    mac.update(s.as_ref());
    let result = mac.finalize().into_bytes().to_vec();
    Ok(result)
}

/// 腾讯云签名函数，签名参考：https://cloud.tencent.com/document/api/228/30978
pub(super) fn sign<P: Serialize, Q: Serialize + Clone>(
    secret_id: &str, secret_key: &str, common_params: &CommonParameter,
    request: &RequestContent<P, Q>, url: &Url,
) -> Result<String, TcCloudError> {

    let canonical_query = serde_qs::to_string(&request.query)?;
    let canonical_headers = format!(
        "content-type:{}\nhost:{}\nx-tc-action:{}\n",
        request.content_type,
        url.host_str().unwrap_or_default(),
        common_params.action.to_lowercase()
    );



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

    let secret_date = hmac_sha256(&date, &format!("TC3{}", secret_key))?;
    let secret_service = hmac_sha256(&common_params.service, &secret_date)?;
    let secret_signing = hmac_sha256(b"tc3_request", &secret_service)?;
    let signature =
        hex::encode(hmac_sha256(&string_to_sign, &secret_signing)?);

    Ok(format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        ALGORITHM, secret_id, credential_scope, SIGNED_HEADERS, signature
    ))
}
