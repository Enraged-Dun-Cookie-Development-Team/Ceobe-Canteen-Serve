use http::Uri;
use url::Url;
pub trait GrpcConfigTrait {
    fn get_url(&self) -> Url;

    fn get_uri(&self) -> Uri {
        let url = self.get_url();
        url.as_str().parse().unwrap()
    }
}
