use std::ops::Deref;
use std::{pin::Pin, sync::Arc};

use crate::mansion::error::{NotFound, MansionError};
use crate::{database::traits::select::LoadFromDb, mansion::modules::mansion};
use futures::Future;
use orm_migrate::DbErr;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter, StreamTrait,
    TransactionTrait,
};

pub struct LoadMansion;

impl LoadFromDb for LoadMansion {
    type Fut = Pin<Box<dyn Future<Output = Result<Self::Target, Self::Err>>>>;

    type Target = mansion::Mansion;

    type Err = MansionError;

    type Args = (u32, u32);

    fn load<'db, Db>(
        (main_id, sub_id): Self::Args,
        db: &Arc<crate::database::ServeDatabase<Db>>,
    ) -> Self::Fut
    where
        Db: ConnectionTrait + TransactionTrait + StreamTrait<'db> + Send + 'static,
    {
        let db = Arc::clone(db);
        Box::pin(async move {
            let db = db.deref();
            let root = super::mansion::Entity::find()
                .filter(
                    Condition::all()
                        .add(super::mansion::Column::Mid.eq(main_id))
                        .add(super::mansion::Column::SubMid.eq(sub_id)),
                )
                .one(db)
                .await?;
            if let Some(rt) = root {
                let id = rt.id;

                let eachs = super::each_mansion::Entity::find()
                    .filter(super::each_mansion::Column::Mid.eq(id))
                    .find_with_related(super::inner_mansion::Entity)
                    .all(db)
                    .await?;

                Ok(Into::<mansion::Mansion>::into((rt, eachs)))
            } else {
                Err(NotFound.into())
            }
        })
    }
}
