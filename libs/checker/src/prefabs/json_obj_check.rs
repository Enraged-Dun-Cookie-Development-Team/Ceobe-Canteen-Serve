use std::{collections::BTreeSet, marker::PhantomData};

use futures::future::{ready, Ready};
use serde_json::Value;
use smallvec::SmallVec;

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
    type Err = JsonObjError;
    type Fut = Ready<Result<String, Self::Err>>;
    type Unchecked = serde_json::Value;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready('check: {
            let Value::Object(ref map) = uncheck else{
                break 'check Err(JsonObjError::NotAObject)
            };

            let mut key_set =
                map.keys().map(String::as_str).collect::<BTreeSet<_>>();

            for require_key in Fields::get_field_iter() {
                if !key_set.remove(require_key) {
                    break 'check Err(JsonObjError::MissField(
                        require_key.to_owned(),
                    ));
                }
            }

            if !key_set.is_empty() {
                let vec = key_set
                    .into_iter()
                    .map(ToOwned::to_owned)
                    .collect::<SmallVec<_>>();
                break 'check Err(JsonObjError::UnexpectedFields(vec));
            }

            Ok(uncheck.to_string())
        })
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]

pub enum JsonObjError {
    #[error("the json value is not a `Object`")]
    NotAObject,
    #[error("require field `{0}` but not found")]
    MissField(String),
    #[error("unexpected fields {0:?}")]
    UnexpectedFields(SmallVec<[String; 2]>),
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use smallvec::SmallVec;

    use super::{JsonObjError, JsonObjectChecker, RequireFields};
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
        let resp =
            JsonObjectChecker::<TestFields>::lite_check(value).into_inner();
        assert_eq!(resp, Err(JsonObjError::MissField("bar".into())))
    }

    #[test]
    fn test_too_many_fields() {
        let value = json! {
            {
                "foo":{
                    "aa":11,
                    "bb":false,
                    "CC":null
                },
                "bar":["a",11,false,{"A":1}],
                "not_me":[1,2,3]
            }
        };
        println!("{:}", value.to_string());
        let resp =
            JsonObjectChecker::<TestFields>::lite_check(value).into_inner();

        assert_eq!(
            resp,
            Err(JsonObjError::UnexpectedFields(SmallVec::from_iter([
                "not_me".into()
            ])))
        )
    }

    #[test]
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
        let resp =
            JsonObjectChecker::<TestFields>::lite_check(value).into_inner();

        assert_eq!(resp, Err(JsonObjError::NotAObject))
    }
}
