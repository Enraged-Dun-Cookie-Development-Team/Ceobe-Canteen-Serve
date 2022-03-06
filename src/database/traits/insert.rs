use std::future::Future;

use crate::database::ServeDatabase;

pub trait SaveToDb<Db> {
    type Fut: Future<Output = Result<Self::Target, Self::Err>>;
    type Target;
    type Err;
    fn insert_into(&self, db: &ServeDatabase<Db>) -> Self::Fut;
}
