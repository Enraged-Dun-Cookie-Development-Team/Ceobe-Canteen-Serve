use std::cmp::Ordering;

use bson::oid::ObjectId;

use crate::CookieId;

impl PartialEq for super::CookieId {
    fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
}

impl PartialEq<ObjectId> for CookieId {
    fn eq(&self, other: &ObjectId) -> bool { self.0.eq(other) }
}

impl PartialEq<CookieId> for ObjectId {
    fn eq(&self, other: &CookieId) -> bool { self.eq(&other.0) }
}

impl PartialEq<str> for super::CookieId {
    fn eq(&self, other: &str) -> bool {
        let Ok(id) = other.parse::<ObjectId>()
        else {
            return false;
        };
        self.0.eq(&id)
    }
}

impl PartialOrd for super::CookieId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd<ObjectId> for CookieId {
    fn partial_cmp(&self, other: &ObjectId) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<str> for CookieId {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.to_string().as_str().partial_cmp(other)
    }
}
