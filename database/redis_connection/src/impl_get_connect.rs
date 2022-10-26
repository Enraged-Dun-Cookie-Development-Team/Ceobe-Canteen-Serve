use std::convert::Infallible;

use database_traits::get_connect::{
    Body, FromRequest, GetMutDatabaseConnect, RequestParts,
};
use redis::aio::ConnectionManager;

use crate::static_var::get_redis_client;

pub struct RedisConnect(ConnectionManager);

impl RedisConnect {
    pub fn from_static() -> Self {
        RedisConnect(get_redis_client().to_owned())
    }
}

impl FromRequest<Body> for RedisConnect {
    type Rejection = Infallible;

    fn from_request<'life0, 'async_trait>(
        _: &'life0 mut RequestParts<Body>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async { Ok(RedisConnect::from_static()) })
    }
}

impl GetMutDatabaseConnect for RedisConnect {
    type Connect<'s> = ConnectionManager
    where
        Self: 's;
    type Error = Infallible;

    fn mut_connect(&mut self) -> Result<&mut Self::Connect<'_>, Self::Error> {
        Ok(&mut self.0)
    }
}
