use std::array::TryFromSliceError;

use bson::oid::ObjectId;
use redis::{FromRedisValue, RedisError, RedisResult, RedisWrite, Value};

impl redis::ToRedisArgs for super::CookieId {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(&self.0.bytes())
    }
}

impl FromRedisValue for super::CookieId {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Data(data) => {
                let buffer: [u8; 12] = data.as_slice().try_into().map_err(
                    |err: TryFromSliceError| {
                        RedisError::from((
                            redis::ErrorKind::TypeError,
                            "Length Not Enough",
                            err.to_string(),
                        ))
                    },
                )?;
                Ok(Self(ObjectId::from(buffer)))
            }
            _ => {
                Err(RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Unsupported CookieId Type",
                )))
            }
        }
    }
}
