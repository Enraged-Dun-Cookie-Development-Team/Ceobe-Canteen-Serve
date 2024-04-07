mod error;
mod sync_newest_cookie;

use redis_connect::RedisConnect;

pub struct SyncCookieOperate<D = RedisConnect>(D);

impl<D> SyncCookieOperate<D> {
    pub fn new(db: D) -> Self { Self(db) }
}

pub type Result<T> = core::result::Result<T, error::Error>;
pub use error::Error;
