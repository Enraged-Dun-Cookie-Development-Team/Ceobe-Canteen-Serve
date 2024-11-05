use std::{borrow::Cow, marker::PhantomData, time::Duration};

use redis::{AsyncCommands, FromRedisValue, RedisResult};

use crate::{redis_value::RedisValue, type_bind::RedisTypeTrait};

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
    T: RedisValue<'redis>,
{
    /// 判定当前值是否存在
    ///
    /// ## 参考
    /// - [`AsyncCommands::exists`]
    pub async fn exists<RV>(&mut self) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.exists(&*self.key).await
    }

    /// 写入当前值
    ///
    /// ## 参考
    /// - [`AsyncCommands::set`]
    pub async fn set<RV>(&mut self, value: T::Input) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.set(&*self.key, value).await
    }

    /// 当值不存在时，写入值
    ///
    /// ## 参考
    /// - [`AsyncCommands::set_nx`]
    pub async fn set_if_not_exist(
        &mut self, value: T::Input,
    ) -> RedisResult<()>
where {
        self.redis.set_nx(&*self.key, value).await
    }

    /// 写入值并添加超时时间
    ///
    /// ## 参考
    /// - [`AsyncCommands::set_ex`]
    pub async fn set_with_expire(
        &mut self, value: T::Input, duration: Duration,
    ) -> RedisResult<()> {
        self.redis
            .set_ex(&*self.key, value, duration.as_secs() as _)
            .await
    }

    /// 获取值
    ///
    /// ## 参考
    /// - [`AsyncCommands::get`]
    pub async fn get(&mut self) -> RedisResult<T::Output> {
        self.redis.get(&*self.key).await
    }

    /// 尝试获取值，如果不存在，返回[`None`]
    ///
    /// ## 参考
    /// - [`AsyncCommands::get`]
    /// - [`AsyncCommands::exists`]
    pub async fn try_get(&mut self) -> RedisResult<Option<T::Output>> {
        Ok(if self.exists().await? {
            Some(self.get().await?)
        }
        else {
            None
        })
    }

    /// 删除值
    ///
    /// ## 参考
    /// - [`AsyncCommands::del`]
    pub async fn remove<RV>(&mut self) -> RedisResult<RV>
    where
        RV: FromRedisValue,
    {
        self.redis.del(&*self.key).await
    }
}
