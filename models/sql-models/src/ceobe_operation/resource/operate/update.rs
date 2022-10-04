use chrono::Local;
use sea_orm::{ConnectionTrait, DbErr};
use sql_connection::database_traits::get_connect::{
    GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
};

use super::{CeobeOperationResourceSqlOperate, OperateError};
use crate::ceobe_operation::resource::{
    checkers::resource_data::CeobeOperationResource,
    models::resource_type::ResourceType,
};

impl CeobeOperationResourceSqlOperate {
    pub async fn update_resource<'db, D>(
        db: &'db D, resource: CeobeOperationResource,
    ) -> Result<(), OperateError>
    where
        D: GetDatabaseConnect<Error = DbErr>,
        D: GetDatabaseTransaction + 'static,
        D::Transaction<'db>: ConnectionTrait,
    {
        let db = db.get_transaction().await?;
        let now = Local::now().naive_local();
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
