use std::{future::Future, pin::Pin};

use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QuerySelect, Select,
};

use super::{Count, COUNT_NAME};

pub trait SelectCount {
    type CountFuture<'db>: Future<Output = Result<Count, sea_orm::DbErr>>
        + 'db
    where
        Self: 'db;
    fn select_count<'db, C, Db>(
        self, col: C, db: &'db Db,
    ) -> Self::CountFuture<'db>
    where
        Self: 'db,
        C: ColumnTrait,
        Db: ConnectionTrait;
}

impl<E: EntityTrait> SelectCount for Select<E> {
    type CountFuture<'db> = Pin<Box<dyn Future<Output = Result<Count,sea_orm::DbErr>>+'db + Send >>
    where
        Self: 'db;

    fn select_count<'db, C, Db>(
        self, col: C, db: &'db Db,
    ) -> Self::CountFuture<'db>
    where
        Self: 'db,
        C: ColumnTrait,
        Db: ConnectionTrait,
    {
        Box::pin(async move {
            Ok(self
                .select_only()
                .column_as(col.count(), COUNT_NAME)
                .into_model::<Count>()
                .one(db)
                .await?
                .unwrap())
        })
    }
}
