mod conv;
mod equal;
mod redis;
mod serde;

use bson::oid::ObjectId;

#[derive(Clone, Copy, Ord, Eq, Debug)]
pub struct CookieId(pub ObjectId);
