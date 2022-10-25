use ceobe_qiniu_upload::SecretConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QiniuSecret {
    access_key: String,
    secret_key: String,
    #[serde(default,alias = "buckets")]
    bucket_list:Vec<String>
}

impl QiniuSecret {
    pub(super) fn get_bucket(&self)->&[String]{
        &self.bucket_list
    }
}

impl SecretConfig for QiniuSecret {
    fn access_key(&self) -> &str { &self.access_key }

    fn secret_key(&self) -> &str { &self.secret_key }
}
