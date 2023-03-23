use core::{future::Future, marker::Send, pin::Pin};
use std::{convert::Infallible, sync::Arc};

use axum_core::extract::{FromRef, FromRequestParts};
use general_request_client::http::request::Parts;
use md5::Digest;
use secrecy::{ExposeSecret, SecretString};
use tokio::sync::{mpsc, oneshot};

use crate::{
    push_models::{
        batch_push_payload::BatchPush, batch_user::BatchUsers, BATCH_SIZE,
    },
    requester::MobPushRequester,
    PushEntity,
};

#[derive(Debug)]
pub struct PushManager {
    push_admission: mpsc::Sender<oneshot::Sender<()>>,
    key: Arc<SecretString>,
    secret: Arc<SecretString>,
    buffer: Vec<u8>,
}

impl<S> FromRequestParts<S> for PushManager
where
    PushManager: FromRef<S>,
    S: Sync,
{
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _: &'life0 mut Parts, state: &'life1 S,
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
        Box::pin(async { Ok(PushManager::from_ref(state)) })
    }
}

impl Clone for PushManager {
    fn clone(&self) -> Self {
        Self {
            push_admission: self.push_admission.clone(),
            key: self.key.clone(),
            secret: self.secret.clone(),
            buffer: Vec::new(),
        }
    }
}

impl PushManager {
    pub(crate) fn new(
        push_admission: mpsc::Sender<oneshot::Sender<()>>,
        key: Arc<SecretString>, secret: Arc<SecretString>, buffer: Vec<u8>,
    ) -> Self {
        Self {
            push_admission,
            key,
            secret,
            buffer,
        }
    }

    pub fn new_requester<'s, 'user, 'string, 'payload, E: PushEntity>(
        &'s mut self, users: &'user [&'string str], content: &'payload E,
    ) -> RequesterIter<'user, 'string, 'payload, 's, BATCH_SIZE, E> {
        RequesterIter {
            buffer: &mut self.buffer,
            users,
            content,
            key: self.key.expose_secret(),
            secret: self.secret.expose_secret(),
        }
    }

    pub fn batch_delay(&self) -> BatchDelayer {
        BatchDelayer {
            inner: self.push_admission.clone(),
        }
    }
}

pub struct BatchDelayer {
    inner: mpsc::Sender<oneshot::Sender<()>>,
}

impl BatchDelayer {
    pub async fn delay(&mut self) {
        let (rx, tx) = oneshot::channel();
        self.inner.send(rx).await.expect("idle thread closed");
        tx.await.ok();
    }
}

pub struct RequesterIter<
    'user,
    'string,
    'payload,
    'manager,
    const BATCH_SIZE: usize,
    E: PushEntity,
> {
    buffer: &'manager mut Vec<u8>,
    users: &'user [&'string str],
    content: &'payload E,
    key: &'manager str,
    secret: &'manager str,
}

impl<
        'user,
        'string,
        'payload,
        'manager,
        const BATCH_SIZE: usize,
        E: PushEntity,
    > Iterator
    for RequesterIter<'user, 'string, 'payload, 'manager, BATCH_SIZE, E>
{
    type Item = Result<MobPushRequester<'manager>, serde_json::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        let Some(users) = BatchUsers::new(&mut self.users)else{
            return None;
        };

        let body = BatchPush::new(users, self.content, self.key);

        if let Err(err) = serde_json::to_writer(&mut self.buffer, &body) {
            return Some(Err(err));
        }

        let mut hasher = <md5::Md5 as Digest>::new();
        hasher.update(&*self.buffer);

        hasher.update(self.secret);

        let md5 = hasher.finalize();
        let md5 = format!("{:x}", md5);

        Some(Ok(MobPushRequester {
            content: self.buffer.clone(),
            key: self.key,
            md5,
        }))
    }
}
