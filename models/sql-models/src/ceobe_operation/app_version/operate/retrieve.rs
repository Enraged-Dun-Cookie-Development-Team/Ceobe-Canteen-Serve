use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};
use sql_connection::get_sql_database;

use super::{
    CeobeOperationAppVersionSqlOperate, OperateError, OperateResult,
};
use crate::ceobe_operation::app_version::models::model_app_version;

impl CeobeOperationAppVersionSqlOperate {
    pub async fn get_app_version_info_by_version(
        version: String,
    ) -> OperateResult<model_app_version::Model> {
        let db = get_sql_database();
        model_app_version::Entity::find()
            .filter(model_app_version::Column::Version.eq(version.clone()))
            .one(db)
            .await?
            .ok_or(OperateError::AppVersionIdNoExist(version))
    }

    pub async fn get_newest_app_version_info(
    ) -> OperateResult<model_app_version::Model> {
        let db = get_sql_database();
        model_app_version::Entity::find()
            .order_by(model_app_version::Column::CreateAt, Order::Desc)
            .one(db)
            .await?
            .ok_or(OperateError::NotAppVersion)
    }
}
