use std::{any::type_name, marker::PhantomData};

use ahash::AHashSet;
use futures::future::{ready, Ready};
use serde::{de::Visitor, Deserialize};

use crate::Checker;

pub trait RequireFields {
    type Iter: IntoIterator<Item = &'static str>;

    fn get_field_iter() -> Self::Iter;
}

#[derive(Debug, Clone, Default)]
pub struct JsonObjectChecker<Fields: RequireFields>(PhantomData<Fields>);

impl<Fields: RequireFields> Checker for JsonObjectChecker<Fields> {
    type Args = ();
    type Checked = String;
    type Err = serde_json::Error;
    type Fut = Ready<Result<String, Self::Err>>;
    type Unchecked = serde_json::Value;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let s = uncheck.to_string();
        ready(serde_json::from_str::<CheckDeserialize<Fields>>(&s).map(|_| s))
    }
}

#[derive(Debug, Clone)]
struct CheckDeserialize<Fields: RequireFields>(PhantomData<Fields>);

impl<'de, Fields: RequireFields> Deserialize<'de>
    for CheckDeserialize<Fields>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CheckerVisiter<Fields: RequireFields>(PhantomData<Fields>);

        impl<Fields: RequireFields> Default for CheckerVisiter<Fields> {
            fn default() -> Self { Self(PhantomData) }
        }

        impl<'de, Fields: RequireFields> Visitor<'de> for CheckerVisiter<Fields> {
            type Value = AHashSet<&'de str>;

            fn expecting(
                &self, formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(
                    formatter,
                    "require map to fit fields[{}]",
                    type_name::<Fields>()
                )
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut vec = AHashSet::with_capacity(
                    map.size_hint().unwrap_or_default(),
                );
                while let Some((key, _)) = map.next_entry::<&str, Any>()? {
                    vec.insert(key);
                }

                Ok(vec)
            }
        }
        let mut key_set = deserializer
            .deserialize_map(CheckerVisiter::<Fields>::default())?;
        for require in Fields::get_field_iter() {
            if !key_set.remove(require) {
                Err(serde::de::Error::missing_field(require))?;
            }
        }
        if !key_set.is_empty() {
            Err(serde::de::Error::custom("fields out of require"))?;
        }
        Ok(Self(PhantomData))
    }
}

struct Any;

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AnyVisitor;

        impl<'de> Visitor<'de> for AnyVisitor {
            type Value = Any;

            fn expecting(
                &self, formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(formatter, "cannot visit")
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_borrowed_bytes<E>(
                self, _: &'de [u8],
            ) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_borrowed_str<E>(
                self, _: &'de str,
            ) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_char<E>(self, _: char) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                let (v, _) = data.variant::<Any>()?;
                Ok(v)
            }

            fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_i128<E>(self, _: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                while map.next_entry::<Any, Any>()?.is_some() {}
                Ok(Any)
            }

            fn visit_newtype_struct<D>(
                self, _: D,
            ) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Any)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                while seq.next_element::<Any>()?.is_some() {}
                Ok(Any)
            }

            fn visit_some<D>(self, _: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Any)
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Any)
            }
        }
        deserializer.deserialize_any(AnyVisitor)
    }
}
#[cfg(test)]
mod test {
    use serde_json::json;

    use super::{JsonObjectChecker, RequireFields};
    use crate::LiteChecker;

    struct TestFields;

    impl RequireFields for TestFields {
        type Iter = [&'static str; 2];

        fn get_field_iter() -> Self::Iter { ["foo", "bar"] }
    }

    #[test]
    fn test_valid() {
        let value = json! {
            {
                "foo":{
                    "aa":11,
                    "bb":false,
                    "CC":null
                },
                "bar":["a",11,false,{"A":1}]
            }
        };
        println!("{:}", value.to_string());
        let _resp = JsonObjectChecker::<TestFields>::lite_check(value)
            .into_inner()
            .expect("Err");
    }

    #[test]
    #[should_panic]
    fn test_miss_fields() {
        let value = json! {
            {
                "foo":{
                    "aa":11,
                    "bb":false
                }
            }
        };
        println!("{:}", value.to_string());
        let _resp = JsonObjectChecker::<TestFields>::lite_check(value)
            .into_inner()
            .expect("Err");
    }

    #[test]
    #[should_panic]
    fn test_too_many_fields() {
        let value = json! {
            {
                "foo":{
                    "aa":11,
                    "bb":false,
                    "CC":null
                },
                "bar":["a",11,false,{"A":1}],
                "www": null
            }
        };
        println!("{:}", value.to_string());
        let _resp = JsonObjectChecker::<TestFields>::lite_check(value)
            .into_inner()
            .expect("Err");
    }

    #[test]
    #[should_panic]
    fn test_is_list() {
        let value = json! {
           [
                {
                    "foo":{
                        "aa":11,
                        "bb":false
                    }
                }
            ]
        };
        println!("{:}", value.to_string());
        let _resp = JsonObjectChecker::<TestFields>::lite_check(value)
            .into_inner()
            .expect("Err");
    }
}
