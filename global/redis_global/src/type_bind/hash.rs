use std::{borrow::Cow, collections::HashMap, marker::PhantomData};

use redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};

use crate::{redis_value::RedisValue, type_bind::RedisTypeTrait};

/// Redis的Hash数据结构类型绑定
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
    T: RedisValue<'redis>,
{
    /// 检查Hash类型中指定field是否存在
    ///
    /// ## 参考
    /// - [`AsyncCommands::hexists`]
    pub async fn exists<'arg, RV, F>(&mut self, field: F) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'arg,
        RV: FromRedisValue,
    {
        self.redis.hexists(&*self.key, field).await
    }

    pub async fn set<'arg, RV, F>(
        &mut self, field: F, value: T::Input,
    ) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'arg,
        RV: FromRedisValue,
    {
        self.redis.hset(&*self.key, field, value).await
    }

    /// 获取当前hash中对应field的对应值
    ///
    /// ## 参考
    /// - [`AsyncCommands::hget`]
    pub async fn get<'arg, F>(&mut self, field: F) -> RedisResult<T::Output>
    where
        F: ToRedisArgs + Send + Sync + 'arg,
    {
        self.redis.hget(&*self.key, field).await
    }

    /// 获取当前hash中对应field的对应值
    ///
    /// ## 参考
    /// - [`AsyncCommands::hall`]
    pub async fn all<K>(&mut self) -> RedisResult<HashMap<K, T::Output>>
    where
        K: FromRedisValue + Eq + std::hash::Hash,
    {
        self.redis.hgetall(&*self.key).await
    }

    /// 尝试获取当前hash中对应的field的对应值，如果不存在，将会返回[`None`]
    ///
    /// ## 参考
    /// - [`AsyncCommands::hexists`]
    /// - [`AsyncCommands::hget`]
    /// - [`Hash::get`]
    /// - [`Hash::exists`]
    pub async fn try_get<'arg, F>(
        &mut self, field: F,
    ) -> RedisResult<Option<T::Output>>
    where
        F: ToRedisArgs + Send + Sync + 'arg + Copy,
    {
        Ok(if self.exists(field).await? {
            Some(self.get(field).await?)
        }
        else {
            None
        })
    }

    /// 尝试删除当前hash中对应的field的对应值
    ///
    /// ## 参考
    /// - [`AsyncCommands::hdel`]
    pub async fn remove<'arg, RV, F>(&mut self, field: F) -> RedisResult<RV>
    where
        F: ToRedisArgs + Send + Sync + 'arg,
        RV: FromRedisValue,
    {
        self.redis.hdel(&*self.key, field).await
    }
}
