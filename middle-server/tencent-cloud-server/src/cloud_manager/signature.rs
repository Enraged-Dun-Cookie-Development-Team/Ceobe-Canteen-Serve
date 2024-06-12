use chrono::DateTime;
use hex::ToHex;
use hmac::{digest::InvalidLength, Hmac, Mac};
use serde::Serialize;
use sha2::{Digest, Sha256};
use url::{Position, Url};

use crate::{
    cloud_manager::entities::{
        CommonParameter, HmacSha256Slice, RequestContent, Sha256HexString,
    },
    error::TcCloudError,
};

/// 签名使用的算法
const ALGORITHM: &str = "TC3-HMAC-SHA256";
/// URI 参数，API 3.0 固定为正斜杠（/）。
const CANONICAL_URI: &str = "/";

/// 参与签名的请求头， 与canonical_headers对应，目前只看到用这三个字段
const SIGNED_HEADERS: &str = "content-type;host;x-tc-action";

/// 计算 [u8] slice 的 [sha256](Sha256) 值，并转换为16进制的 [String]
fn sha256hex(s: &impl AsRef<[u8]>) -> Sha256HexString {
    let mut hasher = Sha256::new();
    hasher.update(s.as_ref());
    let result = hasher.finalize();
    result.encode_hex()
}

fn hmac_sha256(
    s: &impl AsRef<[u8]>, key: &impl AsRef<[u8]>,
) -> Result<HmacSha256Slice, InvalidLength> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key.as_ref())?;
    mac.update(s.as_ref());
    let result = mac.finalize().into_bytes();
    Ok(HmacSha256Slice::from_slice(&result))
}

/// 腾讯云签名函数，签名参考：https://cloud.tencent.com/document/api/228/30978
pub(super) fn gen_signature<P: Serialize, Q: Serialize + Clone>(
    secret_id: &str, secret_key: &str, common_params: &CommonParameter,
    request: &RequestContent<P, Q>, url: &Url,
) -> Result<String, TcCloudError> {
    let canonical_headers = format!(
        "content-type:{}\nhost:{}\nx-tc-action:{}\n",
        request.content_type,
        &url[Position::BeforeHost..Position::AfterHost],
        common_params.action.to_lowercase()
    );

    let canonical_query = serde_qs::to_string(&request.query)?;
    let serialized_payload = serde_json::to_string(&request.payload)?;
    let hashed_payload = sha256hex(&serialized_payload);
    
    // hashing payload
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        request.method,
        CANONICAL_URI,
        canonical_query,
        canonical_headers,
        SIGNED_HEADERS,
        hashed_payload
    );
    let hashed_canonical_request = sha256hex(&canonical_request);

    let datetime =
        DateTime::from_timestamp(common_params.timestamp, 0).unwrap();
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    let credential_scope =
        format!("{}/{}/tc3_request", formatted_date, common_params.service);
    let signature_content = format!(
        "{}\n{}\n{}\n{}",
        ALGORITHM,
        common_params.timestamp,
        credential_scope,
        hashed_canonical_request
    );

    let secreted_date =
        hmac_sha256(&formatted_date, &format!("TC3{}", secret_key))?;
    let secreted_service =
        hmac_sha256(&common_params.service, &secreted_date)?;
    let signature_key = hmac_sha256(b"tc3_request", &secreted_service)?;
    let signature =
        hex::encode(hmac_sha256(&signature_content, &signature_key)?);

    Ok(format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        ALGORITHM, secret_id, credential_scope, SIGNED_HEADERS, signature
    ))
}

#[cfg(test)]
mod test {
    use url::{Position, Url};

    #[test]
    fn test_url_get_host() {
        let url: Url = "http://www.example.com/user/112/profile?from=home"
            .parse()
            .expect("Url Parser Error");
        assert_eq!(
            &url[Position::BeforeHost..Position::AfterHost],
            "www.example.com"
        )
    }
}
