use qq_channel_warning::GrpcConfigTrait;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct QqChannelConfig {
    pub(crate) uri: Url,
}

impl GrpcConfigTrait for QqChannelConfig {
    fn get_url(&self) -> Url { self.uri.clone() }
}
