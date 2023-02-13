use axum::Json;
use ceobe_user_logic::{view::MobIdReq, implements::CeobeUserLogic};
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use crate::router::CeobeUserFrontend;

use super::error::{CeobeUserRResult, CeobeUserError};
impl CeobeUserFrontend {
    /// 获取平台与数据源类型列表
    #[instrument(ret, skip(db, mongo))]
    pub async fn register(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate, MapReject(mob_id): MapReject<Json<MobIdReq>, CeobeUserError>,
    ) -> CeobeUserRResult<()> {
        rtry!(
            CeobeUserLogic::create_user(mongo, db, mob_id).await
        );
        Ok(()).into()
    }
}