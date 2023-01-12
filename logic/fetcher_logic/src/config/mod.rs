use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum_core::extract::{FromRef, FromRequestParts};
use futures::future::ok;

use crate::error::PrepareError;

const SCHEDULE_DOMAIN: &str =
    "config-update.internal.schedule.cookie-fetcher";

pub trait FetcherLogicConfig {
    fn schedule_host(&self) -> IpAddr;

    fn schedule_port(&self) -> u16;
}

#[derive(Debug, Clone)]
pub struct ScheduleNotifier {
    path: Arc<url::Url>,
    client: reqwest::Client,
}

impl<S> FromRequestParts<S> for ScheduleNotifier
where
    ScheduleNotifier: FromRef<S>,
{
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _: &'life0 mut sql_models::sql_connection::database_traits::get_connect::Parts,
        state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(ok(<Self as FromRef<S>>::from_ref(state)))
    }
}

impl ScheduleNotifier {
    pub(crate) fn new(
        c: &impl FetcherLogicConfig,
    ) -> Result<Self, PrepareError> {
        let client = reqwest::ClientBuilder::new()
            .resolve(
                SCHEDULE_DOMAIN,
                SocketAddr::from((c.schedule_host(), 80)),
            )
            .build()?;
        let path = url::Url::try_from(
            format!(
                "http://{}:{}/update-config",
                SCHEDULE_DOMAIN,
                c.schedule_port()
            )
            .as_str(),
        )?;
        Ok(Self {
            path: Arc::new(path),
            client,
        })
    }

    pub(crate) async fn notify_schedule(&self, platform: impl AsRef<str>) {
        self.client
            .post((&*self.path).to_owned())
            .query(&[("platform", platform.as_ref())])
            .send()
            .await
            .ok();
    }
}
