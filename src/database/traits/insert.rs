use std::{future::Future, sync::Arc};

use sea_orm::{ConnectionTrait, StreamTrait, TransactionTrait};

use crate::database::ServeDatabase;

pub trait SaveToDb {
    type Fut: Future<Output = Result<Self::Target, Self::Err>>;
    type Target;
    type Err;
    fn insert_into<'db, Db>(&self, db: &Arc<ServeDatabase<Db>>) -> Self::Fut
    where
        Db: ConnectionTrait
            + TransactionTrait
            + StreamTrait<'db>
            + Send
            + 'static;
}
