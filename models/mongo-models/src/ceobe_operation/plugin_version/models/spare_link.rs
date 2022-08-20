use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpareLink {
    pub url: Url,
    pub msg: String,
}

impl SpareLink {
    pub fn into_tuple(self) -> (String, String) {
        (self.url.to_string(), self.msg)
    }
}
