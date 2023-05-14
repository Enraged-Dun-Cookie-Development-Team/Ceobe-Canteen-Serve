use general_request_client::{
    traits::{RequestBuilder, Requester},
    Method,
};
use url::Url;

use super::{NotifyPath, NotifyRequester};

/// 向调度器发送消息， 平台更新
pub struct NotifyPlatformUpdate<'query> {
    url: Url,
    query: [(&'query str, &'query str); 1],
}

impl<'query> NotifyRequester for NotifyPlatformUpdate<'query> {
    type Args = &'query str;

    fn create<'args>(url: Url, args: Self::Args) -> Self {
        Self::new(url, args)
    }
}

impl<'query> NotifyPath for NotifyPlatformUpdate<'query> {
    const PATH: &'static str = "/schedular-update-config";
}

impl<'query> NotifyPlatformUpdate<'query> {
    fn new(url: Url, platform: &'query str) -> Self {
        Self {
            url,
            query: [("platform", platform)],
        }
    }
}

impl<'query> Requester for NotifyPlatformUpdate<'query> {
    const METHOD: Method = Method::POST;
    const VERSION: http::Version = http::Version::HTTP_11;

    fn get_url(&self) -> Url { self.url.to_owned() }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder.query(&self.query).build()
    }
}
