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
    /// 构造一个Redis 类型绑定， 当key构造需要2个及以上个参数
    fn bind_with_args<'redis, R>(
        &self, redis: &'redis mut R, args: <Self as RedisKey>::Args<'_>,
    ) -> Self::RedisType<'redis, R>
    where
        R: 'redis,
    {
        let key = RedisKey::get_key_with_args(self, args);
        RedisTypeTrait::from_redis_and_key(redis, key)
    }
    /// 构造一个Redis 类型绑定， 当key构造需要一个参数
    fn bind_with<'redis, R>(
        &self, redis: &'redis mut R,
        arg: <<Self as RedisKey>::Args<'_> as RedisKeyArg1>::Arg0,
    ) -> Self::RedisType<'redis, R>
    where
        for<'r> <Self as RedisKey>::Args<'r>: RedisKeyArg1,
    {
        RedisTypeBind::bind_with_args(
            self,
            redis,
            <<Self as RedisKey>::Args<'_> as RedisKeyArg1>::to_this(arg),
        )
    }

    /// 构造一个Redis 类型绑定， 当key构造不需要任何参数
    fn bind<'redis, R>(
        &self, redis: &'redis mut R,
    ) -> Self::RedisType<'redis, R>
    where
        R: 'redis,
        for<'r> <Self as RedisKey>::Args<'r>: RedisKayAutoConstruct,
    {
        RedisTypeBind::bind_with_args(
            self,
            redis,
            RedisKayAutoConstruct::construct(),
        )
    }
}

pub trait RedisKey {
    type Args<'r>;

    #[allow(unused_variables)]
    fn get_key_with_args(&self, arg: Self::Args<'_>) -> Cow<'static, str>;

    fn get_key(&self) -> Cow<'static, str>
    where
        for<'r> Self::Args<'r>: RedisKayAutoConstruct,
    {
        RedisKey::get_key_with_args(self, RedisKayAutoConstruct::construct())
    }
}

pub trait RedisKeyArg1 {
    type Arg0;

    fn to_this(arg0: Self::Arg0) -> Self;
}

impl<T> RedisKeyArg1 for (T,) {
    type Arg0 = T;

    fn to_this(arg0: Self::Arg0) -> Self { (arg0,) }
}

pub trait RedisKayAutoConstruct {
    fn construct() -> Self;
}

impl RedisKayAutoConstruct for () {
    fn construct() -> Self {}
}

macro_rules! redis_key {
    (hash $name:ident::<$t:ty> => $format_key:literal[$($arg:ident:$ty:ident),*])=>{
        #[doc=concat!(concat!("Redis Hash类型绑定\n ## Key \n", $format_key), concat!("\n ## Value Type \n ", stringify!($t)))]
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ($(&'r $ty,)*);

            fn get_key_with_args(&self, args: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
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
        #[doc=concat!(concat!("Redis Hash类型绑定\n ## Key \n", $key), concat!("\n ## Value Type \n ", stringify!($t)))]
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ();

            fn get_key_with_args(&self, _: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
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
        #[doc=concat!(concat!("Redis 普通类型绑定\n ## Key \n", $format_key), concat!("\n ## Value Type \n ", stringify!($t)))]
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ($(&'r $ty,)*);

            fn get_key_with_args(&self, args: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
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
        #[doc=concat!(concat!("Redis 普通类型绑定\n ## Key \n", $key), concat!("\n ## Value Type \n ", stringify!($t)))]
        pub struct $name;

        impl $crate::type_bind::RedisKey for $name {
            type Args<'r> = ();

            fn get_key_with_args(&self, _: Self::Args<'_>) -> std::borrow::Cow<'static, str> {
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