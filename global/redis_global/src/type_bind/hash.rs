use std::{borrow::Cow, marker::PhantomData};

use redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};

use crate::type_bind::RedisTypeTrait;

pub struct Hash<'redis, R: 'redis, T> {
    redis: &'redis mut R,
    key: Cow<'static, str>,
    __phantom: PhantomData<T>,
}

impl<'redis, R, T> RedisTypeTrait<'redis, R> for Hash<'redis, R, T> {
    fn from_redis_and_key(
        redis: &'redis mut R, key: Cow<'static, str>,
    ) -> Self {
        Self {
            redis,
            key,
            __phantom: PhantomData,
        }
    }
}

impl<'redis, R, T> Hash<'redis, R, T>
where
    R: redis::aio::ConnectionLike + Send + Sync,
    T: FromRedisValue + ToRedisArgs + Send + Sync + 'redis,
{
    pub async fn exists<RV, F>(&mut self, field: F) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.hexists(&*self.key, field).await
    }

    pub async fn set<RV, F>(&mut self, field: F, value: T) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.hset(&*self.key, field, value).await
    }

    pub async fn get<F>(&mut self, field: F) -> RedisResult<T>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
    {
        self.redis.hget(&*self.key, field).await
    }

    pub async fn try_get<F>(&mut self, field: F) -> RedisResult<Option<T>>
    where
        F: ToRedisArgs + Send + Sync + 'redis + Copy,
    {
        Ok(if self.exists(field).await? {
            Some(self.get(field).await?)
        }
        else {
            None
        })
    }

    pub async fn remove<RV, F>(&mut self, field: F) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'redis,
        RV: FromRedisValue,
    {
        self.redis.hdel(&*self.key, field).await
    }
}
