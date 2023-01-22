use chrono::Local;
use sea_orm::{ConnectionTrait, DbErr};
use sql_connection::database_traits::get_connect::{
    GetDatabaseTransaction, TransactionOps,
};
use tracing::{info, instrument};

use super::{OperateError, ResourceOperate};
use crate::ceobe_operation::resource::{
    checkers::resource_data::CeobeOperationResource,
    models::resource_type::ResourceType,
};

impl<'op, C> ResourceOperate<'op, C>
where
    C: GetDatabaseTransaction<Error = DbErr>,
    C::Transaction<'op>: ConnectionTrait,
{
    #[instrument(
        skip_all, ret,
        fields(
            resource.all_available = ?resource.resource_all_available,
            resource.countdown.len = resource.countdown.as_ref().map(Vec::len)
        )
    )]
    pub async fn update_resource(
        &'op self, resource: CeobeOperationResource,
    ) -> Result<(), OperateError> {
        let db = self.get_transaction().await?;
        let now = Local::now().naive_local();

        info!(
            updateResource.allAvailable = ?resource.resource_all_available,
            updateResource.countdown.size = resource.countdown.as_deref().map(<[_]>::len)
        );

        // soft remove old resource
        if resource.countdown.is_some() {
            Self::soft_remove(&db, now, ResourceType::Countdown).await?;
        }

        if resource.resource_all_available.is_some() {
            Self::soft_remove(&db, now, ResourceType::ResourceAllAvailable)
                .await?;
        }

        // upload all
        Self::create_new_resource_set(&db, resource, now).await?;

        db.submit().await?;
        Ok(())
    }
}
