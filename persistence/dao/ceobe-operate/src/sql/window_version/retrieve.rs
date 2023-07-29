use std::ops::Deref;

use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{
        ColumnTrait, ConnectionTrait, EntityTrait, Order, QueryFilter,
        QueryOrder,
    },
    tap::TapFallible,
};
use tracing::{info, instrument};

use super::{
    WindowVersionOperate, Column, Entity, Model, OperateError, OperateResult,
};

impl<'c, C> WindowVersionOperate<'c, C>
where
    C: GetDatabaseConnect + 'c,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self, version), ret, fields(version = version.as_ref()))]
    pub async fn get_info_by_version(
        &self, version: &impl AsRef<str>,
    ) -> OperateResult<Model> {
        info!(window.version = version.as_ref());
        Entity::find()
            .filter(Column::Version.eq(version.as_ref()))
            .one(self.deref())
            .await?
            .ok_or_else(|| {
                OperateError::WindowVersionIdNoExist(version.as_ref().to_owned())
            })
    }

    #[instrument(skip(self), ret)]
    pub async fn get_newest_info(&self) -> OperateResult<Model> {
        Entity::find()
            .order_by(Column::CreateAt, Order::Desc)
            .one(self.deref())
            .await?
            .ok_or(OperateError::NotWindowVersion)
            .tap_ok(|version| info!(newestVersion.version = version.version))
    }
}
