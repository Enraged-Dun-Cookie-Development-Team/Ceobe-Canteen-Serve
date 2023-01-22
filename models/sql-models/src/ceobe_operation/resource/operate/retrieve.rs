use futures::{future::join, StreamExt, TryStreamExt};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
    QuerySelect, StreamTrait,
};
use sql_connection::database_traits::get_connect::{
    GetDatabaseTransaction, TransactionOps,
};
use tap::Pipe;
use tracing::{info, instrument};

use super::{OperateError, ResourceOperate};
use crate::{
    ceobe_operation::resource::models::{
        model_resource::{self, Column, Countdown, ResourceAllAvailable},
        resource_type::ResourceType,
    },
    get_zero_data_time,
};

impl<C> ResourceOperate<'_, C> {
    #[instrument(ret, skip_all)]
    pub async fn get_resource_all_available<'db, D>(
        db: &'db D,
    ) -> Result<ResourceAllAvailable, OperateError>
    where
        D: ConnectionTrait + StreamTrait,
    {
        // finding raa
        let mut resp_stream = model_resource::Entity::find()
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
            .into_model::<ResourceAllAvailable>()
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
    ) -> Result<Vec<Countdown>, OperateError>
    where
        D: ConnectionTrait + StreamTrait,
    {
        let resp_stream = model_resource::Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ty.eq(ResourceType::Countdown))
                    .add(Column::DeleteAt.eq(get_zero_data_time())),
            )
            .into_model::<Countdown>()
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
    pub async fn get<F, T>(
        & 'op self, map: F,
    ) -> Result<T, OperateError>
    where
        F: FnOnce(ResourceAllAvailable, Vec<Countdown>) -> T,
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
