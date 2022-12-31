use ceobe_qiniu_upload::{GetBucket, SecretConfig, BaseUrl};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct QiniuUploadConfig {
    access_key: String,
    secret_key: String,
    bucket: String,
    #[serde(alias="url")]
    base_url: Url,
}

impl BaseUrl for QiniuUploadConfig {
    fn get_base_url(&self)->Url {
        self.base_url.clone()
    }
}

impl GetBucket for QiniuUploadConfig {
    fn get_bucket(&self) -> &str {
        &self.bucket
    }
}

impl SecretConfig for QiniuUploadConfig {
    fn access_key(&self) -> &str {
        &self.access_key
    }

    fn secret_key(&self) -> &str {
        &self.secret_key
    }
}

