use core::{future::Future, marker::Send, pin::Pin};
use std::{
    convert::Infallible,
    fmt::{Debug, Formatter},
};

use database_traits::get_connect::{
    FromRequestParts, GetMutDatabaseConnect, Parts,
};
use redis::aio::ConnectionManager;

use crate::static_var::get_redis_client;

#[derive(Clone)]
pub struct RedisConnect(ConnectionManager);

impl Debug for RedisConnect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisConnect").finish()
    }
}

impl RedisConnect {
    pub fn from_static() -> Self {
        RedisConnect(get_redis_client().to_owned())
    }
}

impl<S> FromRequestParts<S> for RedisConnect {
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _parts: &'life0 mut Parts, _state: &'life1 S,
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
        Box::pin(async { Ok(RedisConnect::from_static()) })
    }
}

impl GetMutDatabaseConnect for RedisConnect {
    type Connect = ConnectionManager;

    fn mut_connect(&mut self) -> &mut Self::Connect { &mut self.0 }
}
