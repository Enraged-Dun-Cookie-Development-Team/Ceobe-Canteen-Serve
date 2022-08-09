use futures::StreamExt;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
    QuerySelect, StreamTrait,
};
use sql_connection::get_sql_transaction;

use super::{CeobeOperationResourceSqlOperate, OperateError};
use crate::{
    ceobe_operation::resource::models::{
        model_resource::{self, Column, Countdown, ResourceAllAvailable},
        resource_type::ResourceType,
    },
    get_zero_data_time,
};

impl CeobeOperationResourceSqlOperate {
    pub async fn get_resource_all_available<'db, D>(
        db: &'db D,
    ) -> Result<ResourceAllAvailable, OperateError>
    where
        D: ConnectionTrait + StreamTrait<'db>,
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

        let data = resp_stream.next().await.ok_or(OperateError::NoneAllAvailable)??;

        // not only one raa
        if resp_stream.next().await.is_some() {
            Err(OperateError::MultiAllAvailable)?;
        }

        Ok(data)
    }

    pub async fn get_all_countdown<'db, D>(
        db: &'db D,
    ) -> Result<Vec<Countdown>, OperateError>
    where
        D: ConnectionTrait + StreamTrait<'db>,
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
            .fold(Result::<_, OperateError>::Ok(Vec::new()), |vec, data| {
                async move {
                    let (mut vec, data) = (vec?, data?);
                    vec.push(data);
                    Ok(vec)
                }
            })
            .await?;

        Ok(resp_stream)
    }

    pub async fn get_resource<F, T>(map: F) -> Result<T, OperateError>
    where
        F: FnOnce(ResourceAllAvailable, Vec<Countdown>) -> T,
    {
        let db = get_sql_transaction().await?;

        let raa = Self::get_resource_all_available(&db).await?;
        let countdown = Self::get_all_countdown(&db).await?;

        db.commit().await?;

        Ok(map(raa, countdown))
    }
}
