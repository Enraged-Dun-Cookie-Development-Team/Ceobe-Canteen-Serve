use redis::RedisError;
use thiserror::Error;
use db_ops_prelude::mongodb::bson;
use db_ops_prelude::StatusErr;

#[derive(Debug, Error, StatusErr)]
pub enum Error{
    #[error("Redis异常: {0}")]
    Redis(#[from] RedisError),

    #[error(transparent)]
    BsonOidErr(#[from] bson::oid::Error),
}