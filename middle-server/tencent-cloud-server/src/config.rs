pub trait TencentConfigTrait {
    fn get_secret_id(&self) -> &str;
    fn get_secret_key(&self) -> &str;
}
