use async_trait::async_trait;
use futures::Future;

use crate::database::{error::DatabaseError, ServeDatabase};

pub trait LoadFromDb {
    type Fut:Future<Output = Result<Self::Target,Self::Err>>;
    type Target;
    type Err;
    type Args;
    fn select_by<Db>(args:Self::Args, db: &ServeDatabase<Db>) -> Self::Fut;
}




