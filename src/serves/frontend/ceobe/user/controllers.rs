use axum::{Json, extract::Query};
use ceobe_user_logic::{view::{MobIdReq, DatasourceConfig}, implements::CeobeUserLogic};
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use crate::router::CeobeUserFrontend;

use super::error::{CeobeUserRResult, CeobeUserError};
impl CeobeUserFrontend {
    /// 新建用户（注册mobid入库）
    #[instrument(ret, skip(db, mongo))]
    pub async fn register(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate, MapReject(mob_id): MapReject<Json<MobIdReq>, CeobeUserError>,
    ) -> CeobeUserRResult<()> {
        rtry!(
            CeobeUserLogic::create_user(mongo, db, mob_id).await
        );
        Ok(()).into()
    }

    /// 获取用户数据源配置
    #[instrument(ret, skip(db, mongo))]
    pub async fn get_datasource_config_by_user(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate, MapReject(mob_id): MapReject<Query<MobIdReq>, CeobeUserError>,
    ) -> CeobeUserRResult<DatasourceConfig> {
        Ok(rtry!(
            CeobeUserLogic::get_datasource_by_user(mongo, db, mob_id).await
        )).into()
    }
}