use chrono::{DateTime, Utc};
use hex::ToHex;
use hmac::{digest::InvalidLength, Hmac, Mac};
use sha2::{Digest, Sha256};
use url::Url;

use crate::{
    cloud_manager::entities::{
        HmacSha256Slice, PayloadBuffer, Sha256HexString,
    },
    error::TcCloudError,
    task_trait::{
        header_fetch::get_required_headers, task_request::TaskRequestTrait,
    },
};

/// URI 参数，API 3.0 固定为正斜杠（/）。
const CANONICAL_URI: &str = "/";

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
pub(super) fn gen_signature<Task>(
    secret_id: &str, secret_key: &str, task: &Task, url: &Url,
    payload: &PayloadBuffer, date: &DateTime<Utc>,
) -> Result<String, TcCloudError>
where
    Task: TaskRequestTrait,
{
    let canonical_headers =
        get_required_headers(task.required_sign_header(), task, url)?;

    let canonical_query = serde_qs::to_string(task.query())?;

    let hashed_payload = sha256hex(&payload);

    // hashing payload
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        Task::METHOD,
        CANONICAL_URI,
        canonical_query,
        canonical_headers.formatted_headers,
        canonical_headers.headers,
        hashed_payload
    );
    let hashed_canonical_request = sha256hex(&canonical_request);

    let formatted_date = date.format("%Y-%m-%d").to_string();

    let credential_scope =
        format!("{}/{}/tc3_request", formatted_date, Task::SERVICE);
    let signature_content = format!(
        "{}\n{}\n{}\n{}",
        Task::ALGORITHM,
        date.timestamp(),
        credential_scope,
        hashed_canonical_request
    );

    let secreted_date =
        hmac_sha256(&formatted_date, &format!("TC3{}", secret_key))?;
    let secreted_service = hmac_sha256(&Task::SERVICE, &secreted_date)?;
    let signature_key = hmac_sha256(b"tc3_request", &secreted_service)?;
    let signature =
        hex::encode(hmac_sha256(&signature_content, &signature_key)?);

    Ok(format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        Task::ALGORITHM,
        secret_id,
        credential_scope,
        canonical_headers.headers,
        signature
    ))
}

#[cfg(test)]
mod test {
    use chrono::DateTime;
    use url::{Position, Url};

    use crate::{
        cloud_manager::{
            entities::{PayloadBuffer, Service},
            signature::gen_signature,
        },
        task_trait::{
            serde_content::Json, task_content::TaskContent,
            task_request::TaskRequestTrait,
        },
    };
    use crate::cloud_manager::entities::ServerVersion;

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
    struct Test;
    impl TaskContent for Test {
        type Payload<'r> = Json<'r, str>;

        fn payload(&self) -> Self::Payload<'_> { Json("Acv") }
    }
    impl TaskRequestTrait for Test {
        const ACTION: &'static str = "Action";
        const VERSION: ServerVersion = ServerVersion::Ver20180606;
        const SERVICE: Service = Service::Cdn;
    }

    #[test]
    fn test_signature() {
        let secret_key = "secret_key";
        let secret_id = "secret_id";
        let datetime = DateTime::from_timestamp(0, 0).unwrap();
        let url = Url::parse("http://www.example.com/abc").unwrap();
        let task = Test;

        let signature = gen_signature(
            secret_id,
            secret_key,
            &task,
            &url,
            &PayloadBuffer::new(),
            &datetime,
        )
        .expect("Error");
        // TODO: 使用腾讯云在线签名验证检查实现
        assert_eq!(signature,"TC3-HMAC-SHA256 Credential=secret_id/1970-01-01/cdn/tc3_request, \
        SignedHeaders=content-type;host;x-tc-action, \
        Signature=be22ca9278c0e59fe3f9abc8bc50c47d84a1f8ded7458d1e5c8b3f25c2f19774")
    }
}
