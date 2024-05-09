use std::marker::PhantomData;

use redis::{
    ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite,
    ToRedisArgs, Value,
};
use serde::{Deserialize, Serialize};

use crate::redis_value::RedisValue;

pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn serde(self) -> serde_json::Result<SerdeJson<T>>
    where
        T: Serialize,
    {
        Ok(SerdeJson(serde_json::to_vec(&self.0)?, PhantomData))
    }

    pub fn inner(self) -> T { self.0 }
}

impl<'redis, T> RedisValue<'redis> for Json<T>
where
    T: for<'de> Deserialize<'de> + Serialize + 'static + Send + Sync,
    Self: Send + Sync + 'redis,
{
    type Input = SerdeJson<T>;
    type Output = Self;
}

pub struct SerdeJson<T>(Vec<u8>, PhantomData<T>);
impl<T: Serialize> ToRedisArgs for SerdeJson<T> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.0.write_redis_args(out)
    }
}

impl<T> FromRedisValue for Json<T>
where
    for<'de> T: Deserialize<'de> + 'static,
{
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let Value::Data(data) = v
        else {
            return Err(redis::RedisError::from((
                ErrorKind::TypeError,
                "Expect Json String",
            )));
        };
        let payload = serde_json::from_slice(data).map_err(|err| {
            RedisError::from((
                ErrorKind::TypeError,
                "Cannot Deserialize Json String",
                err.to_string(),
            ))
        })?;

        Ok(Self(payload))
    }
}
