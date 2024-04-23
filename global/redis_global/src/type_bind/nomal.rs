use std::borrow::Cow;

use redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};

use crate::type_bind::RedisTypeTrait;

pub struct Nomral<'redis, R> {
    redis: &'redis mut R,
    key: Cow<'static, str>,
}

impl<'redis, R> RedisTypeTrait<'redis, R> for Nomral<'redis, R> {
    fn from_redis_and_key(
        redis: &'redis mut R, key: Cow<'static, str>,
    ) -> Self {
        Self { redis, key }
    }
}

impl<'redis, R> Nomral<'redis, R>
where
    R: redis::aio::ConnectionLike + Send + Sync,
{
    pub async fn exists<RV>(&mut self) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.exists(&*self.key).await
    }

    pub async fn set<RV, IV>(&mut self, value: IV) -> RedisResult<RV>
    where
        IV: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.set(&*self.key, value).await
    }

    pub async fn get<RV, IV>(&mut self) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.get(&*self.key).await
    }

    pub async fn remove<RV>(&mut self) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.del(&*self.key).await
    }
}
