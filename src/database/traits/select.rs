use std::sync::Arc;

use futures::Future;
use sea_orm::{ConnectionTrait, StreamTrait, TransactionTrait};

use crate::database::ServeDatabase;

pub trait LoadFromDb {
    type Fut: Future<Output = Result<Self::Target, Self::Err>>;
    type Target;
    type Err;
    type Args;
    fn load<'db, Db>(args: Self::Args, db: &Arc<ServeDatabase<Db>>) -> Self::Fut
    where
        Db: ConnectionTrait + TransactionTrait + StreamTrait<'db> + Send + 'static;
}
