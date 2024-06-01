use std::{convert::Infallible, future::Future, pin::Pin, sync::Arc};

use axum_core::extract::{FromRef, FromRequestParts};
use general_request_client::{client::RequestClient, http::request::Parts};
use secrecy::SecretString;


pub struct PartCloudManagerState {
    key: Arc<SecretString>,
    secret: Arc<SecretString>,
}

impl PartCloudManagerState {
    pub(crate) fn new(
        key: Arc<SecretString>, secret: Arc<SecretString>,
    ) -> Self {
        Self {
            key,
            secret,
        }
    }
}


pub struct CloudManager { 
    key: Arc<SecretString>,
    secret: Arc<SecretString>,
    client: RequestClient,
}

impl CloudManager {
    pub fn new_from_state(
        PartCloudManagerState {
            key,
            secret,
        }: PartCloudManagerState,
        client: RequestClient,
    ) -> Self {
        Self {
            key,
            secret,
            client,
        }
    }
}

impl<S> FromRequestParts<S> for CloudManager
where
    PartCloudManagerState: FromRef<S>,
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
            Ok(CloudManager::new_from_state(
                PartCloudManagerState::from_ref(state),
                RequestClient::from_ref(state),
            ))
        })
    }
}