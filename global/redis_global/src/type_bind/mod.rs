use std::borrow::Cow;

pub use crate::type_bind::{hash::Hash, normal::Normal};

mod hash;
mod normal;

pub trait RedisTypeTrait<'redis, R>: Sized {
    fn from_redis_and_key(
        redis: &'redis mut R, key: Cow<'static, str>,
    ) -> Self;

    fn clear(self) { drop(self) }
}

pub trait RedisTypeBind: RedisKey {
    type RedisType<'redis, R>: RedisTypeTrait<'redis, R>
    where
        R: 'redis;

    fn redis_type_with_args<'redis, R>(
        &self, redis: &'redis mut R, args: <Self as RedisKey>::Args<'_>,
    ) -> Self::RedisType<'redis, R>
    where
        R: 'redis,
    {
        let key = RedisKey::get_key(self, args);
        RedisTypeTrait::from_redis_and_key(redis, key)
    }

    fn redis_type<'redis, R>(
        &self, redis: &'redis mut R,
    ) -> Self::RedisType<'redis, R>
    where
        R: 'redis,
        for<'r> <Self as RedisKey>::Args<'r>: RedisKayAutoConstruct,
    {
        RedisTypeBind::redis_type_with_args(
            self,
            redis,
            RedisKayAutoConstruct::construct(),
        )
    }
}

pub trait RedisKey {
    type Args<'r>;

    #[allow(unused_variables)]
    fn get_key(&self, arg: Self::Args<'_>) -> Cow<'static, str>;
}

pub trait RedisKayAutoConstruct {
    fn construct() -> Self;
}

impl RedisKayAutoConstruct for () {
    fn construct() -> Self {}
}

macro_rules! redis_key {
    (hash $name:ident::<$t:ty> => $format_key:literal[$($arg:ident:$ty:ident),*])=>{
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ($(&'r $ty,)*);

            fn get_key(&self, args: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
                let ($($arg,)*) = args;

                (format!($format_key, $($arg),*)).into()
            }
        }

        impl $crate::type_bind::RedisTypeBind for $name {
            type RedisType<'redis, R> = $crate::type_bind::Hash<'redis, R, $t>
                where
                    R: 'redis;
        }
    };
    (hash $name:ident::<$t:ty> => $key:literal) => {
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ();

            fn get_key(&self, _: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
                ($key).into()
            }
        }

        impl $crate::type_bind::RedisTypeBind for $name {
            type RedisType<'redis, R> = $crate::type_bind::Hash<'redis, R, $t>
                where
                    R: 'redis;
        }
    };

     ($name:ident::<$t:ty> => $format_key:literal[$($arg:ident:$ty:ident),*])=>{
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ($(&'r $ty,)*);

            fn get_key(&self, args: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
                let ($($arg,)*) = args;

                (format!($format_key, $($arg),*)).into()
            }
        }

        impl $crate::type_bind::RedisTypeBind for $name {
            type RedisType<'redis, R> = $crate::type_bind::Normal<'redis, R, $t>
                where
                    R: 'redis;
        }
    };
    ($name:ident::<$t:ty> => $key:literal) => {
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ();

            fn get_key(&self, _: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
                ($key).into()
            }
        }

        impl $crate::type_bind::RedisTypeBind for $name {
            type RedisType<'redis, R> = $crate::type_bind::Normal<'redis, R, $t>
                where
                    R: 'redis;
        }
    };
}
