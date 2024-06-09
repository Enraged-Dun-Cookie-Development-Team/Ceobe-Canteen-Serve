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
    /// 请求头
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

    fn get_method(&self) -> Method { self.method.clone() }

    fn get_url(&self) -> Url { self.url.clone() }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder
            .query(&self.query)
            .header(|map| {
                map.append(
                    HOST,
                    HeaderValue::from_str(
                        self.url.host_str().unwrap_or_default(),
                    )
                    .unwrap(),
                );
                map.append(
                    "X-TC-Action",
                    HeaderValue::from_str(&self.action).unwrap(),
                );
                map.append(
                    "X-TC-Version",
                    HeaderValue::from_str(&self.version).unwrap(),
                );
                map.append(
                    "X-TC-Timestamp",
                    HeaderValue::from_str(&self.timestamp.to_string())
                        .unwrap(),
                );
                map.append(
                    CONTENT_TYPE,
                    HeaderValue::from_str(&self.content_type).unwrap(),
                );
                map.append(
                    AUTHORIZATION,
                    HeaderValue::from_str(&self.authorization).unwrap(),
                );
                if let Some(region) = &self.region {
                    map.append(
                        "X-TC-Region",
                        HeaderValue::from_str(region).unwrap(),
                    );
                }
                if let Some(token) = &self.token {
                    map.append(
                        "X-TC-Token",
                        HeaderValue::from_str(token).unwrap(),
                    );
                }
            })
            .body(self.payload)
            .build()
    }
}
