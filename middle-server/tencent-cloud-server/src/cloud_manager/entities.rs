use std::fmt::{Display, Formatter};

use http::HeaderValue;
use serde::Deserialize;
use smallstr::SmallString;
use smallvec::SmallVec;
use typed_builder::TypedBuilder;
use url::Url;

pub type Sha256HexString = SmallString<[u8; 64]>;
pub type HmacSha256Slice = SmallVec<[u8; 32]>;

pub type PayloadBuffer = SmallVec<[u8; 32]>;

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

#[derive(Debug, Clone, Copy)]
pub enum Service {
    Cdn,
}

impl AsRef<[u8]> for Service {
    fn as_ref(&self) -> &[u8] { self.name().as_ref() }
}

impl Display for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Service {
    fn name(&self) -> &'static str {
        match self {
            Service::Cdn => "cdn",
        }
    }

    pub(crate) fn to_url(self) -> Result<Url, url::ParseError> {
        format!("https://{}.tencentcloudapi.com", self.name()).parse()
    }
}

pub enum ServerVersion {
    // 2018-06-06
    Ver20180606,
}

impl ServerVersion {
    fn version(&self) -> &'static str {
        match self {
            ServerVersion::Ver20180606 => "2018-06-06",
        }
    }

    pub fn header_value(&self) -> HeaderValue {
        HeaderValue::from_static(self.version())
    }
}
