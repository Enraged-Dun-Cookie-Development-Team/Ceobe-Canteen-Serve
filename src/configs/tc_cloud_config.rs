use serde::Deserialize;
use tencent_cloud_server::config::TencentConfigTrait;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct TcCloudConfig {
    pub(crate) secret_id: String,
    pub(crate) secret_key: String,
    pub(crate) cdn: Cdn,
}

#[derive(Debug, Deserialize)]
pub struct Cdn {
    pub(crate) url: Url,
}

impl TencentConfigTrait for TcCloudConfig {
    fn get_secret_id(&self) -> &str { &self.secret_id }

    fn get_secret_key(&self) -> &str { &self.secret_key }

    fn get_cdn_base_url(&self) -> Url { self.cdn.url.clone() }
}
