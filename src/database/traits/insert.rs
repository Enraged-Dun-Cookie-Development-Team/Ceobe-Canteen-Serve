use async_trait::async_trait;

use crate::database::{error::DatabaseError, ServeDatabase};

#[async_trait]
pub trait InsertInto<Db> {
    type Target;
    async fn insert_into(&self, db: &ServeDatabase<Db>) -> Result<Self::Target, DatabaseError>;
}
