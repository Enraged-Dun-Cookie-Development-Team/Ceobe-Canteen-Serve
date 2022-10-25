pub trait SecretConfig {
    fn access_key(&self) -> &str;
    fn secret_key(&self) -> &str;
}
