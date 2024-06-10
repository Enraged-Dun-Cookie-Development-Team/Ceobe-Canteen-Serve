use general_request_client::{
    http::header::{AUTHORIZATION, CONTENT_TYPE, HOST},
    traits::{RequestBuilder, Requester},
    HeaderValue, Method, Url, Version,
};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct TencentCloudRequester<T: Serialize> {
    /// 请求链接
    pub(crate) url: Url,
    /// 请求方法
    pub(crate) method: Method,
    /// 请求参数
    pub(crate) query: T,

    /// 请求内容
    /// FIXME: 使用基于泛型的接口绑定
    pub(crate) payload: Vec<u8>,
    pub(crate) host: HeaderValue,
    pub(crate) action: HeaderValue,
    pub(crate) version: HeaderValue,
    pub(crate) timestamp: HeaderValue,
    pub(crate) content_type: HeaderValue,
    pub(crate) authorization: HeaderValue,
    pub(crate) region: Option<HeaderValue>,
    pub(crate) token: Option<HeaderValue>,
}

impl<T: Serialize> Requester for TencentCloudRequester<T> {
    const METHOD: Method = Method::POST;
    const VERSION: Version = Version::HTTP_11;

    fn get_method(&self) -> Method { self.method.clone() }

    fn get_url(&self) -> Url { self.url.clone() }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder
            .query(&self.query)
            .header(|map| {
                map.append(HOST, self.host.clone());
                map.append("X-TC-Action", self.action.clone());
                map.append("X-TC-Version", self.version.clone());
                map.append("X-TC-Timestamp", self.timestamp.clone());
                map.append(CONTENT_TYPE, self.content_type.clone());
                map.append(AUTHORIZATION, self.authorization.clone());
                if let Some(region) = &self.region {
                    map.append("X-TC-Region", region.clone());
                }
                if let Some(token) = &self.token {
                    map.append("X-TC-Token", token.clone());
                }
            })
            .body(self.payload)
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
