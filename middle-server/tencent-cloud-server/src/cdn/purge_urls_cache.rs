use serde::Serialize;

use crate::{
    cloud_manager::CloudManager,
    common_parameters::{CommonParameter, RequestContent, TcCloudResponse},
    error::TcCloudError,
};

#[derive(Debug, Clone, Serialize)]
struct PurgeUrlsCache {
    #[serde(rename = "Urls")]
    urls: Vec<String>,
}

impl CloudManager {
    pub async fn purge_urls_cache(
        &self, paths: Vec<&str>,
    ) -> Result<TcCloudResponse, TcCloudError> {
        let urls = paths
            .into_iter()
            .map(|path| format!("{}{}", self.cdn_base_url, path))
            .collect();
        let payload = PurgeUrlsCache { urls };

        let common_params = CommonParameter::builder()
            .service("cdn".to_string())
            .version("2018-06-06".to_string())
            .action("PurgeUrlsCache".to_string())
            .build();
        let request = RequestContent::builder()
            .payload(payload)
            .content_type("application/json; charset=utf-8".to_string())
            .query("".to_string())
            .build();

        Self::common_request(self, &common_params, &request).await
    }
}
