use axum::{extract::Query, Json};
use ceobe_user_logic::{
    implements::CeobeUserLogic,
    view::{DatasourceConfig, MobIdReq},
};
use checker::{CheckExtract, JsonCheckExtract};
use mongo_migration::{
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::user_property::check::user_checker::UserPropertyChecker,
};
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use super::error::{CeobeUserError, CeobeUserRResult};
use crate::router::CeobeUserFrontend;
impl CeobeUserFrontend {
    /// 新建用户（注册mobid入库）
    #[instrument(ret, skip(db, mongo))]
    pub async fn register(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        MapReject(mob_id): MapReject<Json<MobIdReq>, CeobeUserError>,
    ) -> CeobeUserRResult<()> {
        rtry!(CeobeUserLogic::create_user(mongo, db, mob_id).await);
        Ok(()).into()
    }

    /// 获取用户数据源配置
    #[instrument(ret, skip(db, mongo))]
    pub async fn get_datasource_config_by_user(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        MapReject(mob_id): MapReject<Query<MobIdReq>, CeobeUserError>,
    ) -> CeobeUserRResult<DatasourceConfig> {
        Ok(rtry!(
            CeobeUserLogic::get_datasource_by_user(mongo, db, mob_id).await
        ))
        .into()
    }

    /// 更新用户数据源配置
    #[instrument(ret, skip(db, mongo))]
    pub async fn update_datasource_config_by_user(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        CheckExtract(user, ..): JsonCheckExtract<UserPropertyChecker, CeobeUserError>,
    ) -> CeobeUserRResult<()> {
        Ok(rtry!(
            CeobeUserLogic::update_datasource(mongo, db, user).await
        ))
        .into()
    }
}
