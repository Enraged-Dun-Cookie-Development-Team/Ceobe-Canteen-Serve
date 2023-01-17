use http::HeaderMap;
use reqwest::ClientBuilder;
use tap::Pipe;

use crate::traits::{RequestBuilder, Requester};

#[derive(Debug, Clone)]
pub struct RequestClient(reqwest::Client);

impl RequestBuilder for reqwest::RequestBuilder {
    type Body = reqwest::Body;

    type Request = reqwest::Request;

    type Error = reqwest::Error;

    fn query<T: serde::Serialize>(self, query: &T) -> Self {
        self.query(query)
    }

    fn header<F: FnMut(&mut http::HeaderMap)>(self, mut editor: F) -> Self {
        let mut map = HeaderMap::new();
        editor(&mut map);
        self.headers(map)
    }

    fn body<T: Into<Self::Body>>(self, body: T) -> Self {
        self.body(body)
    }

    fn json<T: serde::Serialize>(self, json: &T) -> Self {
        self.json(json)
    }

    fn form<T: serde::Serialize>(self, form: &T) -> Self {
        self.form(form)
    }

    fn build(self) -> Result<Self::Request, Self::Error> {
        self.build()
    }
}

impl RequestClient {
    pub async fn send_request<Q: Requester>(
        &self, requester: Q,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = requester.get_url();
        let builder = self.0.request(Q::METHOD, url).version(Q::VERSION);

        let request = requester.prepare_request(builder)?;

        let resp = self.0.execute(request).await?;

        Ok(resp)
    }
}

impl RequestClient {
    pub(crate) fn new_with(
        cfg: impl FnOnce(ClientBuilder) -> ClientBuilder,
    ) -> Result<Self, reqwest::Error> {
        reqwest::Client::builder().pipe(cfg).build().map(Self)
    }
}
