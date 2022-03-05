use std::future::Future;
use async_trait::async_trait;

use crate::database::{error::DatabaseError, ServeDatabase};

pub trait SaveToDb<Db> {
    type Fut:Future<Output = Result<Self::Target,Self::Err>>;
    type Target;
    type Err;
    fn insert_into(&self, db: &ServeDatabase<Db>) -> Self::Fut;
}
