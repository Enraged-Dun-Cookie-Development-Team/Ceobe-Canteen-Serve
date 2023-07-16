use std::{convert::Infallible, sync::Arc};

use axum_core::extract::{FromRef, FromRequestParts};
use futures::{future::ok, Future};
use general_request_client::{
    client::RequestClient,
    traits::{ClientResult, Requester},
};
use http::request::Parts;
use url::Url;

use crate::{
    config::SchedulerNotifierConfig,
    requesters::notify_platform_update::NotifyPlatformUpdate, NotifyPath,
    NotifyRequester,
};

#[derive(Debug, Clone)]
pub struct SchedulerUrl(Arc<Url>);

impl SchedulerUrl {
    pub fn new(url: Url) -> Self { Self(Arc::new(url)) }

    pub fn new_cfg(cfg: &impl SchedulerNotifierConfig) -> Self {
        Self::new(cfg.base_url())
    }

    pub fn take_url(&self) -> Url { Url::clone(&self.0) }
}

pub type SchedulerNotifier = NotifySender;

pub struct NotifySender {
    base_url: Arc<Url>,
    client: RequestClient,
}

impl<S> FromRequestParts<S> for NotifySender
where
    SchedulerUrl: FromRef<S>,
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
            base_url: SchedulerUrl::from_ref(state).0,
            client: FromRef::from_ref(state),
        }))
    }
}

impl NotifySender {
    pub fn new(base_url: Url, client: RequestClient) -> Self {
        Self {
            base_url: Arc::new(base_url),
            client,
        }
    }

    pub async fn send_notify<N>(
        &self, args: N::Args,
    ) -> ClientResult<RequestClient>
    where
        N: NotifyRequester + Requester + NotifyPath,
    {
        let requester =
            N::create_with_base_url(Url::clone(&self.base_url), args);

        self.client.send_request(requester).await
    }

    pub async fn notify_platform_update(&self, platform: impl AsRef<str>) {
        self.send_notify::<NotifyPlatformUpdate>(platform.as_ref())
            .await
            .ok();
    }
}
