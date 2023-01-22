use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait};
use tracing::info;

use super::ResourceOperate;
use crate::ceobe_operation::resource::{
    checkers::resource_data::CeobeOperationResource, models::model_resource,
};

impl<C> ResourceOperate<'_,C> {
    pub async fn create_new_resource_set(
        db: &impl ConnectionTrait, resource: CeobeOperationResource,
        now: NaiveDateTime,
    ) -> Result<(), super::OperateError> {
        info!(
            newResource.allAvailable = ?resource.resource_all_available,
            newResource.countdown.size =
                resource.countdown.as_deref().map(<[_]>::len)
        );

        let (raa, cd) = resource.into_active_list(now);
        if let Some(raa) = raa {
            raa.insert(db).await?;
        }

        if !cd.is_empty() {
            model_resource::Entity::insert_many(cd).exec(db).await?;
        }

        Ok(())
    }
}
