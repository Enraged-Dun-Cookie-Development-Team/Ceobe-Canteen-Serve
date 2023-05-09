use core::{future::Future, marker::Send, pin::Pin};
use std::convert::Infallible;

use database_traits::get_connect::{
    FromRequestParts, GetDatabaseCollection, GetDatabaseConnect, Parts,
};
use serde::{Deserialize, Serialize};

use crate::{
    static_vars::{get_mongo_collection, get_mongo_database},
    CollectionGuard, DatabaseManage, MongoDbError,
};

#[derive(Debug, Clone, Default)]
pub struct MongoConnect;

impl<S: Send + Sync> FromRequestParts<S> for MongoConnect {
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
        Box::pin(async { Ok(MongoConnect) })
    }
}

impl GetDatabaseConnect for MongoConnect {
    type Connect = DatabaseManage;

    fn get_connect(&self) -> &Self::Connect { get_mongo_database() }
}

impl<C> GetDatabaseCollection<C> for MongoConnect
where
    C: Serialize + for<'de> Deserialize<'de> + 'static,
{
    type CollectGuard<'s> = CollectionGuard<C>;
    type Error = MongoDbError;

    fn get_collection(&self) -> Result<Self::CollectGuard<'_>, Self::Error> {
        get_mongo_collection()
    }
}

pub trait MongoDbCollectionTrait<'db, T>:
    GetDatabaseConnect
    + GetDatabaseCollection<
        T,
        Error = MongoDbError,
        CollectGuard<'db> = CollectionGuard<T>,
    >
where
    Self: 'static,
    T: Serialize + for<'de> Deserialize<'de> + 'static,
{
}

impl<'db, D, T> MongoDbCollectionTrait<'db, T> for D
where
    T: Serialize + for<'de> Deserialize<'de> + 'static,
    Self: GetDatabaseConnect
        + GetDatabaseCollection<
            T,
            Error = MongoDbError,
            CollectGuard<'db> = CollectionGuard<T>,
        > + 'static,
{
}
