use async_trait::async_trait;

use crate::database::{error::DatabaseError, ServeDatabase};

#[async_trait]
pub trait SelectBy<Db> {
    type Target;
    async fn select_by(&self, db: &ServeDatabase<Db>) -> Result<Self::Target, DatabaseError>;
}


