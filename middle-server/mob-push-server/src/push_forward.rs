use std::collections::BTreeMap;

use general_request_client::Url;
use serde::ser::SerializeStruct;
use serde_json::Value;
use typed_builder::TypedBuilder;

use crate::push_notify::NotifySerialize;

#[derive(Debug)]
pub enum PushForward {
    HomePage,
    Link(Url),
    Scheme(Scheme),
    Internet(Url),
}

impl PushForward {
    pub fn new_to_home_page() -> Self { Self::HomePage }

    pub fn new_to_link(url: Url) -> Self { Self::Link(url) }

    pub fn new_to_scheme(scheme: Scheme) -> Self { Self::Scheme(scheme) }

    pub fn new_to_internet(url: Url) -> Self { Self::Internet(url) }
}

impl NotifySerialize for PushForward {
    fn serialize_field(&self) -> usize {
        match self {
            PushForward::HomePage => 1,
            PushForward::Link(_) => 2,
            PushForward::Scheme(Scheme { uri, value, .. }) => {
                2 + usize::from(uri.is_none()) + usize::from(value.is_none())
            }
            PushForward::Internet(_) => 2,
        }
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        struct_serialize.serialize_field(
            "nextType",
            &match self {
                PushForward::HomePage => 0,
                PushForward::Link(_) => 1,
                PushForward::Scheme(_) => 2,
                PushForward::Internet(_) => 3,
            },
        )?;

        match self {
            PushForward::HomePage => (),
            PushForward::Link(url) => {
                struct_serialize.serialize_field("url", url)?
            }
            PushForward::Scheme(Scheme { scheme, uri, value }) => {
                struct_serialize.serialize_field("scheme", scheme)?;
                if let Some(url) = uri {
                    struct_serialize.serialize_field("url", url)?;
                }

                if let Some(map) = value {
                    struct_serialize.serialize_field(
                        "schemeDataList",
                        &map.iter().collect::<Vec<_>>(),
                    )?;
                }
            }
            PushForward::Internet(url) => {
                struct_serialize.serialize_field("intentUrl", url)?
            }
        };

        Ok(())
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Scheme {
    scheme: Url,
    uri: Option<Url>,
    value: Option<BTreeMap<String, Value>>,
}
