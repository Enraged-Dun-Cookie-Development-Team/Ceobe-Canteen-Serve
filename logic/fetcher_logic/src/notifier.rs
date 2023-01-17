use core::{future::Future, marker::Send};
use std::{convert::Infallible, sync::Arc};

use axum_core::extract::{FromRef, FromRequestParts};
use futures::future::ok;
use general_request_client::{
    client::RequestClient,
    traits::{RequestBuilder, Requester},
    Method,
};
use sql_models::sql_connection::database_traits::get_connect::Parts;
use url::Url;

use crate::config;

#[derive(Debug, Clone)]
pub struct FetcherNotifyScheduleUrl(Arc<Url>);

impl FetcherNotifyScheduleUrl {
    pub fn new(url: Url) -> Self { Self(Arc::new(url)) }

    pub fn new_cfg(
        cfg: &impl config::FetcherLogicConfig,
    ) -> Result<Self, url::ParseError> {
        let url = url::Url::try_from(
            format!(
                "http://{}:{}/update-config",
                cfg.schedule_host(),
                cfg.schedule_port()
            )
            .as_str(),
        )?;
        Ok(Self::new(url))
    }
}

pub type ScheduleNotifier = NotifySender;

pub struct NotifySender {
    base_url: Arc<Url>,
    client: RequestClient,
}

impl<S> FromRequestParts<S> for NotifySender
where
    FetcherNotifyScheduleUrl: FromRef<S>,
    RequestClient: FromRef<S>,
{
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _parts: &'life0 mut Parts, state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn Future<Output = Result<Self, Self::Rejection>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(ok(Self {
            base_url: FetcherNotifyScheduleUrl::from_ref(state).0,
            client: FromRef::from_ref(state),
        }))
    }
}

impl NotifySender {
    pub async fn notify_schedule(&self, platform: impl AsRef<str>) {
        let requester = NotifyRequest::new(&self.base_url, platform.as_ref());
        let _ = self.client.send_request(requester).await.ok();
    }
}

struct NotifyRequest<'url, 'query> {
    url: &'url Url,
    query: [(&'query str, &'query str); 1],
}

impl<'url, 'query> NotifyRequest<'url, 'query> {
    pub fn new(url: &'url Url, platform: &'query str) -> Self {
        Self {
            url,
            query: [("platform", platform)],
        }
    }
}

impl<'url, 'query> Requester for NotifyRequest<'url, 'query> {
    const METHOD: Method = Method::POST;

    fn get_url(&self) -> Url { self.url.to_owned() }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder.query(&self.query).build()
    }
}
