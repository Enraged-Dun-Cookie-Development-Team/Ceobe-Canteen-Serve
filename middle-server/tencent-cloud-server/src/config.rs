use general_request_client::Url;


pub trait TencentConfigTrait {
    fn get_secret_id(&self) -> &str;
    fn get_secret_key(&self) -> &str;
    fn get_cdn_base_url(&self) -> Url;
}
