use qq_channel_warning::GrpcConfigTrait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QqChannelConfig {
    pub(crate) uri: String,
}

impl GrpcConfigTrait for QqChannelConfig {
    fn get_uri(&self) -> &str {
        &self.uri
    }
}
