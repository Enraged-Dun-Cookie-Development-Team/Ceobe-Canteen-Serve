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
        let inner = String::from_redis_value(v)?.parse().map_err(
            |err: bson::oid::Error| {
                RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Bad ObjectId Format",
                    err.to_string(),
                ))
            },
        )?;
        Ok(Self(inner))
    }
}
