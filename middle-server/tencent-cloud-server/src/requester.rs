use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use general_request_client::{
    traits::{RequestBuilder, Requester},
    Method, Url, Version,
};
use http::HeaderMap;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::{
    cloud_manager::entities::PayloadBuffer,
    error::TcCloudError,
    task_trait::{
        header_fetch::{
            to_header_map, Authorization, ContentType, HeaderFetch, Host,
            TcAction, TcRegion, TcTimestamp, TcToken, TcVersion,
        },
        task_request::TaskRequestTrait,
    },
};

#[derive(Debug, TypedBuilder)]
pub struct TencentCloudRequester<'t, T, Q> {
    /// 请求链接
    pub(crate) url: Url,
    pub(crate) query: &'t Q,
    pub(crate) payload: PayloadBuffer,
    pub(crate) header_map: HeaderMap,
    #[builder(default = PhantomData, setter(skip))]
    __phantom: PhantomData<&'t T>,
}

impl<T> TencentCloudRequester<'_, T, ()> {
    pub fn new<'t>(
        task: &'t T, url: Url, authorization: &str, date_time: DateTime<Utc>,
        serialized_payload: PayloadBuffer,
    ) -> Result<TencentCloudRequester<'t, T, impl Serialize + 't>, TcCloudError>
    where
        T: TaskRequestTrait,
    {
        let header_map = to_header_map(
            [
                &Host as &dyn HeaderFetch<T>,
                &TcAction,
                &TcVersion,
                &TcTimestamp(&date_time),
                &ContentType,
                &Authorization(authorization),
            ]
            .into_iter()
            .chain(T::REGION.map(|_| &TcRegion as _))
            .chain(T::TOKEN.map(|_| &TcToken as _)),
            task,
            &url,
        )?;
        Ok(TencentCloudRequester::<T, _>::builder()
            .payload(serialized_payload)
            .query(task.query())
            .url(url)
            .header_map(header_map)
            .build())
    }
}

impl<'t, T: TaskRequestTrait, Q: Serialize> Requester
    for TencentCloudRequester<'t, T, Q>
{
    const METHOD: Method = T::METHOD;
    const VERSION: Version = Version::HTTP_11;

    fn get_url(&self) -> Url { self.url.clone() }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder
            .query(self.query)
            .header(move |map| map.extend(self.header_map))
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
