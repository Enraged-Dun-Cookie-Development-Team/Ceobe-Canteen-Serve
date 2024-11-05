use axum::Json;
use axum_resp_result::{rtry, MapReject};
use ceobe_user_logic::{
    implements::CeobeUserLogic,
    view::{DatasourceCombResp, DatasourceConfig, MobIdReq},
};
use mob_push_server::PushManager;
use persistence::{
    ceobe_user::{models::models::UserDatasource, ToCeobe, ToCeobeUser},
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
    redis::RedisConnect,
};
use tracing::instrument;

use super::error::{CeobeUserError, CeobeUserRResult};
use crate::{middleware::mob::MobIdInfo, router::CeobeUserFrontend};

impl CeobeUserFrontend {
    /// 新建用户（注册mobid入库）
    #[instrument(ret, skip(db, mongo))]
    pub async fn register(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        mob: PushManager,
        MapReject(mob_id): MapReject<Json<MobIdReq>, CeobeUserError>,
    ) -> CeobeUserRResult<()> {
        rtry!(CeobeUserLogic::create_user(mongo, db, mob, mob_id).await);
        Ok(()).into()
    }

    /// 获取用户数据源配置
    #[instrument(ret, skip(db, mongo, redis_client))]
    pub async fn get_datasource_config_by_user(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        redis_client: RedisConnect, MobIdInfo(mob_id): MobIdInfo,
    ) -> CeobeUserRResult<DatasourceConfig> {
        Ok(rtry!(
            CeobeUserLogic::get_datasource_by_user(
                mongo,
                db,
                redis_client,
                mob_id
            )
            .await
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

    // 获取用户数据源配置
    #[instrument(ret, skip(db, mongo, redis_client))]
    pub async fn get_comb_by_datasources(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        redis_client: RedisConnect,
        MapReject(datasource_config): MapReject<
            Json<UserDatasource>,
            CeobeUserError,
        >,
    ) -> CeobeUserRResult<DatasourceCombResp> {
        Ok(rtry!(
            CeobeUserLogic::get_comb_by_datasources(
                mongo,
                db,
                redis_client,
                datasource_config.datasource_push
            )
            .await
        ))
        .into()
    }
}
