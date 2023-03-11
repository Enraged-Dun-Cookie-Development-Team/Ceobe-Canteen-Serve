use abstract_database::ceobe::ToCeobe;
use axum::Json;
use ceobe_qiniu_upload::QiniuUploader;
use ceobe_user::ToCeobeUser;
use ceobe_user_logic::{
    implements::CeobeUserLogic,
    view::{DatasourceConfig, MobIdReq},
};
use mongo_migration::{
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::user_property::models::UserDatasource,
};
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use super::error::{CeobeUserError, CeobeUserRResult};
use crate::{middleware::mob::MobIdInfo, router::CeobeUserFrontend};
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
    #[instrument(ret, skip(db, mongo, uploader))]
    pub async fn get_datasource_config_by_user(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate, uploader: QiniuUploader, 
        MobIdInfo(mob_id): MobIdInfo,
    ) -> CeobeUserRResult<DatasourceConfig> {
        Ok(rtry!(
            CeobeUserLogic::get_datasource_by_user(mongo, db, uploader, mob_id).await
        ))
        .into()
    }

    /// 更新用户数据源配置
    #[instrument(ret, skip(db, mongo))]
    pub async fn update_datasource_config_by_user(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        MobIdInfo(mob_id): MobIdInfo,
        MapReject(datasource_config): MapReject<
            Json<UserDatasource>,
            CeobeUserError,
        >,
    ) -> CeobeUserRResult<()> {
        rtry!(
            CeobeUserLogic::update_datasource(
                mongo,
                db,
                datasource_config.datasource_push,
                mob_id
            )
            .await
        );
        Ok(()).into()
    }

    /// 更新用户最后活跃时间
    #[instrument(ret, skip(mongo))]
    pub async fn update_user_access_time(
        mongo: MongoDatabaseOperate, MobIdInfo(mob_id): MobIdInfo,
    ) -> CeobeUserRResult<()> {
        Ok(rtry!(
            mongo
                .ceobe()
                .user()
                .property()
                .update_access_time(mob_id.mob_id)
                .await
        ))
        .into()
    }
}
