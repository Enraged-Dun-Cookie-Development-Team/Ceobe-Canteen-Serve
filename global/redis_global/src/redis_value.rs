use redis::{FromRedisValue, ToRedisArgs};

use crate::CookieId;

pub trait RedisValue<'redis> {
    type Input: ToRedisArgs + Send + Sync + 'redis;

    type Output: FromRedisValue + Send + Sync + 'redis;
}

macro_rules! impl_redis_value_for_base_type {
    ($t:ty) => {
        impl<'redis> RedisValue<'redis> for $t{
            type Input = Self;
            type Output = Self;
        }
    };
    [$($t:ty),*]=>{
        $(
            impl_redis_value_for_base_type!($t);
        )*
    }
}

impl_redis_value_for_base_type![
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, bool, String,
    CookieId
];
