use db_ops_prelude::{
    chrono::NaiveDateTime,
    sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait},
};
use tracing::info;

use super::{Checked, Entity, OperateResult, ResourceOperate};

impl<C> ResourceOperate<'_, C> {
    pub async fn create_new_resource_set(
        db: &impl ConnectionTrait, resource: Checked, now: NaiveDateTime,
    ) -> OperateResult<()> {
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
            Entity::insert_many(cd).exec(db).await?;
        }

        Ok(())
    }
}
