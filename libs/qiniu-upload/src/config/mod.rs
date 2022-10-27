pub trait SecretConfig {
    fn access_key(&self) -> &str;
    fn secret_key(&self) -> &str;
}

/// 实现获取Bucket
pub trait GetBucket {
    type BucketName: AsRef<str> + ?Sized;
    type Iterator<'i>: Iterator<Item = &'i Self::BucketName>
    where
        Self: 'i;

    fn get_buckets(&self) -> Self::Iterator<'_>;
}
