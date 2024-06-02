use general_request_client::{traits::{RequestBuilder, Requester}, HeaderValue, Method, Url, Version};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct TencentCloudRequester<T: Serialize> {
    /// 请求链接
    pub(crate) url: String,
    /// 请求方法
    pub(crate) method: Method,
    /// 请求参数
    pub(crate) query: T,
    /// 请求内容
    pub(crate) payload: Vec<u8>,
    /// 请求头
    pub(crate) host: String,
    pub(crate) action: String,
    pub(crate) version: String,
    pub(crate) timestamp: i64,
    pub(crate) content_type: String,
    pub(crate) authorization: String,
    pub(crate) region: Option<String>,
    pub(crate) token: Option<String>,
}

impl<T: Serialize> Requester for TencentCloudRequester<T> {
    const METHOD: Method = Method::POST;
    const VERSION: Version = Version::HTTP_11;

    fn get_method(&self) -> Method {
        self.method.clone()
    }

    fn get_url(&self) -> Url {
        self.url.parse().unwrap()
    }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder
            .query(&self.query)
            .header(|map| {
                map.append("Host", HeaderValue::from_str(&self.host).unwrap());
                map.append("X-TC-Action", HeaderValue::from_str(&self.action).unwrap());
                map.append("X-TC-Version", HeaderValue::from_str(&self.version).unwrap());
                map.append("X-TC-Timestamp", HeaderValue::from_str(&self.timestamp.to_string()).unwrap());
                map.append("Content-Type", HeaderValue::from_str(&self.content_type).unwrap());
                map.append("Authorization", HeaderValue::from_str(&self.authorization).unwrap());
                if self.region.is_some() {
                    map.append("X-TC-Region", HeaderValue::from_str(self.region.as_ref().unwrap()).unwrap());
                }
                if self.token.is_some() {
                    map.append("X-TC-Token", HeaderValue::from_str(self.token.as_ref().unwrap()).unwrap());
                }
            })
            .body(self.payload)
            .build()
    }
}
