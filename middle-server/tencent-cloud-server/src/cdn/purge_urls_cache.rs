use general_request_client::Url;
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
        let request = RequestContent::<_, ()>::builder()
            .payload(payload)
            .content_type("application/json; charset=utf-8".parse().unwrap())
            .build();

        Self::common_request(self, &common_params, &request).await
    }
}

#[cfg(test)]
mod test {
    use general_request_client::Method;
    use mime::Mime;
    use serde::Serialize;
    use typed_builder::TypedBuilder;

    #[derive(Debug, Clone, TypedBuilder)]
    pub struct RequestContent<P: Serialize, Q: Serialize + Clone> {
        #[builder(default = Method::POST)]
        pub method: Method,
        pub payload: P,
        pub query: Q,
        pub content_type: Mime,
    }

    #[test]
    fn test_serde_qs() {
        let request = RequestContent::builder()
            .payload("")
            .content_type("application/json; charset=utf-8".parse().unwrap())
            .query(Option::<String>::None)
            .build();
        let canonical_query =
            serde_qs::to_string(&request.query).expect("序列化发生错误");

        println!("{}", canonical_query);
    }
}
