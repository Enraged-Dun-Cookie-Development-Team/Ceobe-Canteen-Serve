pub trait SecretConfig {
    fn access_key(&self) -> &str;
    fn secret_key(&self) -> &str;
}

/// 实现获取Bucket
pub trait GetBucket {
    fn get_bucket(&self) -> &str;
}
