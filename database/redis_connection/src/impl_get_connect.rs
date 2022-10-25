use std::convert::Infallible;

use database_traits::get_connect::{FromRequest, Body, RequestParts, GetDatabaseConnect};
use redis::{RedisError, Connection, Client};

use crate::static_var::{get_redis_client, get_redis_connection};


#[derive(Debug, Default)]
pub struct RedisConnect;

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
        Box::pin(async { Ok(RedisConnect) })
    }
}

impl GetDatabaseConnect for RedisConnect {
    type Connect<'s> = Connection;
    type Error = RedisError;

    fn get_connect(&self) -> Result<&Self::Connect<'_>, Self::Error> {
       get_redis_connection()
    }
}