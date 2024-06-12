use general_request_client::{
    http::header::{AUTHORIZATION, CONTENT_TYPE, HOST},
    traits::{RequestBuilder, Requester},
    HeaderValue, Method, Url, Version,
};
use http::HeaderMap;
use typed_builder::TypedBuilder;

use crate::{
    cloud_manager::entities::PayloadBuffer,
    task_trait::task_request::TaskRequestTrait,
};

#[derive(Debug, Clone, TypedBuilder)]
pub struct TencentCloudRequester<'t, T: TaskRequestTrait> {
    /// 请求链接
    pub(crate) url: Url,
    pub(crate) task: &'t T,
    pub(crate) payload: PayloadBuffer,
    pub(crate) host: HeaderValue,
    pub(crate) action: HeaderValue,
    pub(crate) version: HeaderValue,
    pub(crate) timestamp: HeaderValue,
    pub(crate) content_type: HeaderValue,
    pub(crate) authorization: HeaderValue,
    pub(crate) region: Option<HeaderValue>,
    pub(crate) token: Option<HeaderValue>,
}

impl<'t, T: TaskRequestTrait> Requester for TencentCloudRequester<'t, T> {
    const METHOD: Method = T::METHOD;
    const VERSION: Version = Version::HTTP_11;

    fn get_url(&self) -> Url { self.url.clone() }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        let mut header_map = HeaderMap::new();
        header_map.append(HOST, self.host);
        header_map.append("X-TC-Action", self.action);
        header_map.append("X-TC-Version", self.version);
        header_map.append("X-TC-Timestamp", self.timestamp);
        header_map.append(CONTENT_TYPE, self.content_type);
        header_map.append(AUTHORIZATION, self.authorization);
        if let Some(region) = self.region {
            header_map.append("X-TC-Region", region);
        }
        if let Some(token) = self.token {
            header_map.append("X-TC-Token", token);
        }

        builder
            .query(self.task.query())
            .header(|map| {
                let update_map = std::mem::take(&mut header_map);
                map.extend(update_map)
            })
            .body(self.payload.to_vec())
            .build()
    }
}

#[cfg(test)]
mod test {
    use general_request_client::Method;
    use mime::Mime;
    use reqwest::Client;
    use serde::Serialize;
    use typed_builder::TypedBuilder;

    #[derive(Debug, Clone, TypedBuilder)]
    pub struct RequestContent<P, Q>
    where
        P: Serialize,
        Q: Serialize + Clone,
    {
        #[builder(default = Method::POST)]
        pub method: Method,
        pub payload: P,
        #[builder(default = Option::<Q>::None, setter(strip_option))]
        pub query: Option<Q>,
        pub content_type: Mime,
    }

    #[test]
    fn test_serde() {
        let _ = Client::new()
            .request(Method::POST, "https://www.baidu.com")
            .query(&Option::<String>::None)
            .build()
            .expect("client构建失败");
    }
}
