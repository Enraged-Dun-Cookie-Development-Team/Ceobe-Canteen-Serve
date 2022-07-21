use mongo_connection::{get_mongo_collection, CollectionGuard, MongoDbError};

use super::preludes::ModelMansion;

mod create;
mod delete;
mod retrieve;
mod update;
mod verify;

pub struct MansionDataMongoOperate;

pub type MongoErr = mongodb::error::Error;

fn get_mansion_collection(
) -> Result<CollectionGuard<ModelMansion>, MongoDbError> {
    get_mongo_collection()
}
