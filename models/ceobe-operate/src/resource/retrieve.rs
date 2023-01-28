use db_ops_prelude::{
    futures::{future::join, StreamExt, TryStreamExt},
    get_connect::{GetDatabaseTransaction, TransactionOps},
    get_zero_data_time,
    sea_orm::{
        ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait,
        QueryFilter, QuerySelect, StreamTrait,
    },
    tap::Pipe,
};
use tracing::{info, instrument};

use super::{
    all_available, countdown, Column, Entity, OperateError, OperateResult,
    ResourceOperate, ResourceType,
};

impl<C> ResourceOperate<'_, C> {
    #[instrument(ret, skip_all)]
    pub async fn get_resource_all_available<'db, D>(
        db: &'db D,
    ) -> OperateResult<all_available::Model>
    where
        D: ConnectionTrait + StreamTrait,
    {
        // finding raa
        let mut resp_stream = Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ty.eq(ResourceType::ResourceAllAvailable))
                    .add(Column::DeleteAt.eq(get_zero_data_time())),
            )
            .select_only()
            .column(Column::StartTime)
            .column(Column::OverTime)
            .column(Column::CreateAt)
            .column(Column::ModifyAt)
            .column(Column::DeleteAt)
            .into_model::<all_available::Model>()
            .stream(db)
            .await?;

        let data = resp_stream
            .next()
            .await
            .ok_or(OperateError::NoneAllAvailable)??;

        // not only one raa
        if resp_stream.next().await.is_some() {
            Err(OperateError::MultiAllAvailable)?;
        }
        Ok(data)
    }

    #[instrument(skip_all)]
    pub async fn get_all_countdown<'db, D>(
        db: &'db D,
    ) -> OperateResult<Vec<countdown::Model>>
    where
        D: ConnectionTrait + StreamTrait,
    {
        let resp_stream = Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ty.eq(ResourceType::Countdown))
                    .add(Column::DeleteAt.eq(get_zero_data_time())),
            )
            .into_model::<countdown::Model>()
            .stream(db)
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        info!(countdown.size = resp_stream.len());
        Ok(resp_stream)
    }
}
impl<'op, C> ResourceOperate<'op, C>
where
    C: GetDatabaseTransaction<Error = DbErr>,
    C::Transaction<'op>: ConnectionTrait + StreamTrait,
{
    #[instrument(
        skip_all,
        fields(
            resource.countdown.len,
            resource.all_available
        )
    )]
    pub async fn get<F, T>(&'op self, map: F) -> OperateResult<T>
    where
        F: FnOnce(all_available::Model, Vec<countdown::Model>) -> T,
    {
        let db = self.get_transaction().await?;
        let (raa, countdown) = join(
            Self::get_resource_all_available(&db),
            Self::get_all_countdown(&db),
        )
        .await
        .pipe(|(raa, countdown)| Ok::<_, OperateError>((raa?, countdown?)))?;

        db.submit().await?;

        Ok(map(raa, countdown))
    }
}
