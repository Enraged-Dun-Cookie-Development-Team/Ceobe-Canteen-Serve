use std::slice::Iter;

use ceobe_qiniu_upload::{GetBucket, SecretConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QiniuUploadConfig {
    access_key: String,
    secret_key: String,
    #[serde(default, alias = "buckets")]
    bucket_list: Vec<String>,
}

impl GetBucket for QiniuUploadConfig {
    type BucketName = str;
    type Iterator<'i> =  std::iter::Map< Iter<'i,String>,fn(&String)->&str>
    where
        Self: 'i;

    fn get_buckets(&self) -> Self::Iterator<'_> {
        self.bucket_list.iter().map(String::as_str)
    }
}

impl SecretConfig for QiniuUploadConfig {
    fn access_key(&self) -> &str { &self.access_key }

    fn secret_key(&self) -> &str { &self.secret_key }
}
