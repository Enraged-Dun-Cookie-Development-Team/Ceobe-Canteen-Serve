use chrono::NaiveDateTime;
use sea_orm::{ConnectionTrait, EntityTrait};

use super::CeobeOperationResourceSqlOperate;
use crate::ceobe_operation::resource::{
    checkers::resource_data::CeobeOperationResource, models::model_resource,
};

impl CeobeOperationResourceSqlOperate {
    pub async fn create_new_resource_set(
        db: &impl ConnectionTrait, resource: CeobeOperationResource,
        now: NaiveDateTime,
    ) -> Result<(), super::OperateError> {
        let actives = resource.into_active_list(now);
        if !actives.is_empty() {
            model_resource::Entity::insert_many(actives)
                .exec(db)
                .await?;
        }

        Ok(())
    }
}
