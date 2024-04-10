use std::cmp::Ordering;
use bson::oid::ObjectId;

impl PartialEq for super::CookieId{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialEq<str> for super::CookieId{
    fn eq(&self, other: &str) -> bool {
        let Ok(id) = other.parse::<ObjectId>()else{
            return false
        };
        self.0.eq(&id)
    }
}

impl PartialOrd for super::CookieId{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}



