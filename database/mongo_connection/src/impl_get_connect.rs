use database_traits::get_connect::{
    GetDatabaseCollection, GetDatabaseConnect,
};
use serde::{Deserialize, Serialize};

use crate::{
    error, get_mongo_collection, get_mongo_database, CollectionGuard,
    DatabaseManage,
};

#[derive(Debug,Default)]
pub struct MongoConnect;

impl GetDatabaseConnect for MongoConnect {
    type Connect<'s> = &'s DatabaseManage;
    type Error = error::MongoDbError;

    fn get_connect(&self) -> Result<Self::Connect<'_>, Self::Error> {
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
