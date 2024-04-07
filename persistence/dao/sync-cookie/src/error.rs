use db_ops_prelude::{mongodb::bson, StatusErr};
use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum Error {
    #[error("Redis异常: {0}")]
    Redis(#[from] RedisError),

    #[error(transparent)]
    BsonOidErr(#[from] bson::oid::Error),
}
