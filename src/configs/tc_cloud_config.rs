use serde::Deserialize;
use tencent_cloud_server::config::TencentConfigTrait;

#[derive(Debug, Deserialize)]
pub struct TcCloudConfig {
    pub(crate) secret_id: String,
    pub(crate) secret_key: String,
    pub(crate) cdn: Cdn,
}

#[derive(Debug, Deserialize)]
pub struct Cdn {
    pub(crate) url: String,
}

impl TencentConfigTrait for TcCloudConfig {
    fn get_secret_id(&self) -> &str { &self.secret_id }

    fn get_secret_key(&self) -> &str { &self.secret_key }

    fn get_cdn_base_url(&self) -> &str { &self.cdn.url }
}
