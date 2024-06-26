use general_request_client::Url;
use serde::Serialize;
use url::Position;

use crate::{
    cloud_manager::{
        entities::{ServerVersion, Service, TencentCloudResponse},
        TencentCloudManager,
    },
    error::TcCloudError,
    task_trait::{
        serde_content::Json, task_content::TaskContent,
        task_request::TaskRequestTrait,
    },
};

const ACTION: &str = "PurgeUrlsCache";

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

#[derive(Debug, Clone, Serialize)]
pub struct PurgeUrlsCache {
    #[serde(rename = "Urls")]
    pub(crate) urls: Vec<Url>,
}

impl TaskContent for PurgeUrlsCache {
    type Payload<'r> = Json<'r, Self>;

    fn payload(&self) -> Self::Payload<'_> { Json(self) }
}

impl TaskRequestTrait for PurgeUrlsCache {
    const ACTION: &'static str = ACTION;
    const SERVICE: Service = Service::Cdn;
    const VERSION: ServerVersion = ServerVersion::Ver20180606;
}

impl PurgeUrlsCache {
    pub fn new<'i>(
        manager: &TencentCloudManager,
        paths: impl IntoIterator<Item = &'i PurgeCachePath>,
    ) -> Self {
        let urls = paths
            .into_iter()
            .map(|PurgeCachePath { path, query }| {
                let mut url = Url::clone(&*manager.cdn_base_url);
                let prefix = &url[Position::BeforePath..];
                url.set_path(&(prefix.to_string() + path));
                url.set_query(query.as_deref());
                url
            })
            .collect();
        Self { urls }
    }
}

impl TencentCloudManager {
    pub async fn purge_urls_cache(
        &self, paths: impl IntoIterator<Item = &PurgeCachePath>,
    ) -> Result<TencentCloudResponse, TcCloudError> {
        self.exec_request(&PurgeUrlsCache::new(self, paths)).await
    }
}

#[cfg(test)]
mod test {
    use general_request_client::Method;
    use mime::Mime;
    use serde::Serialize;
    use typed_builder::TypedBuilder;
    use url::{Position, Url};

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

    #[test]
    fn test_url() {
        let mut url =
            Url::parse("http://server-cdn-dev.ceobecanteen.top/api/v1")
                .unwrap();

        let prefix = &url[Position::BeforePath..];
        url.set_path(&(prefix.to_string() + "/test/test"));
        println!("{}", url)
    }
}
