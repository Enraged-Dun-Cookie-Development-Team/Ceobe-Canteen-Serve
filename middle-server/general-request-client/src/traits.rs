use http::{HeaderMap, Method, Version};
use serde::Serialize;
use url::Url;
pub trait Requester: Sized {
    const METHOD: Method;
    const VERSION: Version = Version::HTTP_2;

    fn get_url(&self) -> Url;

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder.build()
    }
}

pub trait RequestBuilder: Sized {
    type Body;
    type Request;
    type Error;

    fn query<T: Serialize>(self, query: &T) -> Self;

    fn header<F: FnMut(&mut HeaderMap)>(self, editor: F) -> Self;

    fn body<T: Into<Self::Body>>(self, body: T) -> Self;

    fn json<T: Serialize>(self, json: &T) -> Self;

    fn form<T: Serialize>(self, form: &T) -> Self;

    fn build(self) -> Result<Self::Request, Self::Error>;
}
