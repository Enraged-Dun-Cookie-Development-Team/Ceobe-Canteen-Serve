use general_request_client::{client::RequestClient, traits::{ClientTrait, Requester}, Method};

pub struct ReqClient(pub RequestClient);

struct MobRequester {
    url:url::Url,
}

impl Requester for MobRequester {
    const METHOD: Method = Method::POST;

    fn get_url(&self) -> reqwest::Url {
        self.url
    }
    fn prepare_request<B: general_request_client::traits::RequestBuilder>(
            self, builder: B,
        ) -> Result<B::Request, B::Error> {
        
    }
}

impl mob_push::http_client::PushClient for ReqClient {
    type RequestBuilder = ReqBuilder;

    type Error = <RequestClient as ClientTrait>::Error;

    fn post(&self, url: impl Into<url::Url>) -> Self::RequestBuilder {
        self.0.new
    }

    fn send_request<'life0,'async_trait>(&'life0 self,req: <Self::RequestBuilder as mob_push::http_client::PushRequestBuilder> ::Request,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result< <Self::RequestBuilder as mob_push::http_client::PushRequestBuilder> ::Response,Self::Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait{
        todo!()
    }
}

pub struct ReqBuilder(reqwest::RequestBuilder);

impl mob_push::http_client::PushRequestBuilder for ReqBuilder {
    type Error = reqwest::Error;

    type Request = reqwest::Request;

    type Response = Response;

    fn header(self, key: &'static str, value: &str) -> Self {
        Self(self.0.header(key, value))
    }

    fn body(self, payload: Vec<u8>) -> Self {
        Self(self.0.body(payload))
    }

    fn build(self) -> Result<Self::Request, Self::Error> {
        self.0.build()
    }
}

pub struct Response(reqwest::Response);

impl mob_push::http_client::PushResponse for Response {
    type Error = reqwest::Error;

    fn status(&self) -> u16 {
        self.0.status().as_u16()
    }

    fn bytes<'async_trait>(
        self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Vec<u8>, Self::Error>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        Box::pin(async {
            let bytes = self.0.bytes().await?;
            Ok(bytes.into())
        })
    }
}
