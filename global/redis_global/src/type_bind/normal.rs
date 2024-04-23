use std::{borrow::Cow, marker::PhantomData};

use redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};

use crate::type_bind::RedisTypeTrait;

pub struct Normal<'redis, R, T> {
    redis: &'redis mut R,
    key: Cow<'static, str>,
    __phantom: PhantomData<T>,
}

impl<'redis, R, T> RedisTypeTrait<'redis, R> for Normal<'redis, R, T> {
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

impl<'redis, R, T> Normal<'redis, R, T>
where
    R: redis::aio::ConnectionLike + Send + Sync,
    T: FromRedisValue + ToRedisArgs + Sync + Send + 'redis,
{
    pub async fn exists<RV>(&mut self) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.exists(&*self.key).await
    }

    pub async fn set<RV>(&mut self, value: T) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.set(&*self.key, value).await
    }

    pub async fn set_nx<RV>(&mut self, value: T) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.set_nx(&*self.key, value).await
    }

    pub async fn set_ex<RV>(
        &mut self, value: T, second: usize,
    ) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.set_ex(&*self.key, value, second).await
    }

    pub async fn get(&mut self) -> RedisResult<T> {
        self.redis.get(&*self.key).await
    }

    pub async fn remove<RV>(&mut self) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.del(&*self.key).await
    }
}
