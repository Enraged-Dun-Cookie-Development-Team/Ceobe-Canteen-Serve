use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::TapFallible;
use tracing::{info, instrument};

use super::{AppVersionOperate, OperateError, OperateResult};
use crate::ceobe_operation::app_version::models::model_app_version;

impl<'c, C> AppVersionOperate<'c, C>
where
    C: GetDatabaseConnect + 'c,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self, version), ret, fields(version = version.as_ref()))]
    pub async fn get_info_by_version(
        &self, version: &impl AsRef<str>,
    ) -> OperateResult<model_app_version::Model> {
        info!(app.version = version.as_ref());
        model_app_version::Entity::find()
            .filter(model_app_version::Column::Version.eq(version.as_ref()))
            .one(self.get_connect())
            .await?
            .ok_or_else(|| {
                OperateError::AppVersionIdNoExist(version.as_ref().to_owned())
            })
    }

    #[instrument(skip(self), ret)]
    pub async fn get_newest_info(
        &self,
    ) -> OperateResult<model_app_version::Model> {
        model_app_version::Entity::find()
            .order_by(model_app_version::Column::CreateAt, Order::Desc)
            .one(self.get_connect())
            .await?
            .ok_or(OperateError::NotAppVersion)
            .tap_ok(|version| info!(newestVersion.version = version.version))
    }
}
