use std::{convert::Infallible, future::Future, pin::Pin, sync::Arc};

use axum_core::extract::{FromRef, FromRequestParts};
use general_request_client::{
    client::RequestClient, http::request::Parts, Url,
};
use secrecy::SecretString;



#[derive(Debug, Clone)]
pub struct PartTencentCloudManagerState {
    id: Arc<SecretString>,
    key: Arc<SecretString>,
    cdn_base_url: Arc<Url>,
}

impl PartTencentCloudManagerState {
    pub(crate) fn new(
        id: Arc<SecretString>, key: Arc<SecretString>, cdn_base_url: Arc<Url>,
    ) -> Self {
        Self {
            id,
            key,
            cdn_base_url,
        }
    }
}

pub struct TencentCloudManager {
    pub(crate) id: Arc<SecretString>,
    pub(crate) key: Arc<SecretString>,
    pub(crate) cdn_base_url: Arc<Url>,
    pub(crate) client: RequestClient,
}

impl TencentCloudManager {
    pub fn new_from_state(
        PartTencentCloudManagerState {
            id,
            key,
            cdn_base_url,
        }: PartTencentCloudManagerState,
        client: RequestClient,
    ) -> Self {
        Self {
            id,
            key,
            cdn_base_url,
            client,
        }
    }
}

impl<S> FromRequestParts<S> for TencentCloudManager
where
    PartTencentCloudManagerState: FromRef<S>,
    RequestClient: FromRef<S>,
    S: Sync,
{
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _parts: &'life0 mut Parts, state: &'life1 S,
    ) -> Pin<
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
        Box::pin(async {
            Ok(TencentCloudManager::new_from_state(
                PartTencentCloudManagerState::from_ref(state),
                RequestClient::from_ref(state),
            ))
        })
    }
}
