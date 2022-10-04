use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, Order, QueryFilter,
    QueryOrder,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;

use super::{
    CeobeOperationAppVersionSqlOperate, OperateError, OperateResult,
};
use crate::ceobe_operation::app_version::models::model_app_version;

impl CeobeOperationAppVersionSqlOperate {
    pub async fn get_app_version_info_by_version<'db, D>(
        db: &'db D, version: &impl AsRef<str>,
    ) -> OperateResult<model_app_version::Model>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        model_app_version::Entity::find()
            .filter(model_app_version::Column::Version.eq(version.as_ref()))
            .one(db.get_connect()?)
            .await?
            .ok_or_else(|| {
                OperateError::AppVersionIdNoExist(version.as_ref().to_owned())
            })
    }

    pub async fn get_newest_app_version_info<'db, D>(
        db: &'db D,
    ) -> OperateResult<model_app_version::Model>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        model_app_version::Entity::find()
            .order_by(model_app_version::Column::CreateAt, Order::Desc)
            .one(db.get_connect()?)
            .await?
            .ok_or(OperateError::NotAppVersion)
    }
}
