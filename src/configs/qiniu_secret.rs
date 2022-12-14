
use ceobe_qiniu_upload::{GetBucket, SecretConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QiniuUploadConfig {
    access_key: String,
    secret_key: String,
    bucket: String,
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
