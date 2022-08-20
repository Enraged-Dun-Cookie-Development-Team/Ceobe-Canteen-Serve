use chrono::Local;
use sql_connection::get_sql_transaction;

use super::{CeobeOperationResourceSqlOperate, OperateError};
use crate::ceobe_operation::resource::{
    checkers::resource_data::CeobeOperationResource,
    models::resource_type::ResourceType,
};

impl CeobeOperationResourceSqlOperate {
    pub async fn update_resource(
        resource: CeobeOperationResource,
    ) -> Result<(), OperateError> {
        let db = get_sql_transaction().await?;
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

        db.commit().await?;
        Ok(())
    }
}
