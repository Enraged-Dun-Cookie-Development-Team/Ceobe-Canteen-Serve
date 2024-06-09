use general_request_client::Url;
use mime::Mime;
use serde::Serialize;

use super::{SERVICE, VERSION};
use crate::{
    cloud_manager::TcCloudManager,
    common_parameters::{CommonParameter, RequestContent, TcCloudResponse},
    error::TcCloudError,
};

const ACTION: &str = "PurgeUrlsCache";

#[derive(Debug, Clone, Serialize)]
struct PurgeUrlsCache {
    #[serde(rename = "Urls")]
    urls: Vec<Url>,
}

pub struct PurgeCachePath {
    path: &'static str,
    query: Option<String>,
}

impl PurgeCachePath {
    pub const fn new(path: &'static str) -> Self {
        PurgeCachePath { path, query: None }
    }

    pub fn new_with_query(
        path: &'static str, query: &impl Serialize,
    ) -> Result<Self, serde_qs::Error> {
        let query_str = serde_qs::to_string(query)?;
        Ok(PurgeCachePath {
            path,
            query: Some(query_str),
        })
    }
}

impl TcCloudManager {
    pub async fn purge_urls_cache(
        &self, paths: impl IntoIterator<Item = &PurgeCachePath>,
    ) -> Result<TcCloudResponse, TcCloudError> {
        let urls = paths
            .into_iter()
            .map(|PurgeCachePath { path, query }| {
                let mut url = Url::clone(&*self.cdn_base_url);
                url.set_path(path);
                url.set_query(query.as_deref());
                url
            })
            .collect();
        let payload = PurgeUrlsCache { urls };

        let common_params = CommonParameter::builder()
            .service(SERVICE)
            .version(VERSION)
            .action(ACTION)
            .build();
        let request = RequestContent::builder()
            .payload(payload)
            .content_type("application/json; charset=utf-8".parse().unwrap())
            .query("")
            .build();

        Self::common_request(self, &common_params, &request).await
    }
}
