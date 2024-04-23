use std::borrow::Cow;

use redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};

use crate::type_bind::RedisTypeTrait;

pub struct Hash<'redis, R:'redis> {
    redis: &'redis mut R,
    key: Cow<'static, str>,
}

impl<'redis, R> RedisTypeTrait<'redis, R> for Hash<'redis, R> {
    fn from_redis_and_key(
        redis: &'redis mut R, key: Cow<'static, str>,
    ) -> Self {
        Self { redis, key }
    }
}

impl<'redis, R> Hash<'redis, R>
where
    R: redis::aio::ConnectionLike + Send + Sync,
{
    pub async fn exists<RV, F>(&mut self, field: F) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.hexists(&*self.key, field).await
    }

    pub async fn set<RV, IV, F>(
        &mut self, field: F, value: IV,
    ) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
        IV: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.hset(&*self.key, field, value).await
    }

    pub async fn get<RV, F>(&mut self, field: F) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.hget(&*self.key, field).await
    }

    pub async fn remove<RV, F>(&mut self, field: F) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.hdel(&*self.key, field).await
    }
}
