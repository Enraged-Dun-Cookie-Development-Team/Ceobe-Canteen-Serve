use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use bson::oid::ObjectId;

use crate::cookie_id::CookieId;

impl From<ObjectId> for CookieId {
    fn from(value: ObjectId) -> Self { Self(value) }
}

impl From<[u8; 12]> for CookieId {
    fn from(value: [u8; 12]) -> Self { ObjectId::from(value).into() }
}

impl FromStr for CookieId {
    type Err = <ObjectId as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ObjectId::from_str(s)?.into())
    }
}

impl Deref for CookieId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl AsRef<ObjectId> for CookieId {
    fn as_ref(&self) -> &ObjectId { &self.0 }
}

impl Display for CookieId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <ObjectId as Display>::fmt(&self.0, f)
    }
}

