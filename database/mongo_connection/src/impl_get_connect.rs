use std::convert::Infallible;

use database_traits::get_connect::{
    Body, FromRequest, GetDatabaseCollection, GetDatabaseConnect,
};
use serde::{Deserialize, Serialize};

use crate::{
    error, CollectionGuard,
    DatabaseManage, MongoDbError, static_vars::{get_mongo_database, get_mongo_collection},
};

#[derive(Debug, Default)]
pub struct MongoConnect;

impl FromRequest<Body> for MongoConnect {
    type Rejection = Infallible;

    fn from_request<'life0, 'async_trait>(
        _: &'life0 mut database_traits::get_connect::RequestParts<Body>,
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
        Box::pin(async { Ok(MongoConnect) })
    }
}

impl GetDatabaseConnect for MongoConnect {
    type Connect<'s> = DatabaseManage;
    type Error = error::MongoDbError;

    fn get_connect(&self) -> Result<&Self::Connect<'_>, Self::Error> {
        Ok(get_mongo_database())
    }
}

impl<C> GetDatabaseCollection<C> for MongoConnect
where
    C: Serialize + for<'de> Deserialize<'de> + 'static,
{
    type CollectGuard<'s> = CollectionGuard<C>;

    fn get_collection(&self) -> Result<Self::CollectGuard<'_>, Self::Error> {
        get_mongo_collection()
    }
}

pub trait MongoDbCollectionTrait<'db, T>:
    GetDatabaseConnect<Error = MongoDbError>
    + GetDatabaseCollection<T, CollectGuard<'db> = CollectionGuard<T>>
where
    Self: 'static,
    T: Serialize + for<'de> Deserialize<'de> + 'static,
{
}

impl<'db, D, T> MongoDbCollectionTrait<'db, T> for D
where
    T: Serialize + for<'de> Deserialize<'de> + 'static,
    Self: GetDatabaseConnect<Error = MongoDbError>
        + GetDatabaseCollection<T, CollectGuard<'db> = CollectionGuard<T>>
        + 'static,
{
}
