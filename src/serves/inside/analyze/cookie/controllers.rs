use axum::Json;
use axum_resp_result::{resp_try, MapReject};
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic, view::NewCookieReq,
};
use ceobe_qiniu_upload::QiniuManager;
use mob_push_server::PushManager;
use persistence::{
    mongodb::MongoDatabaseOperate, mysql::SqlDatabaseOperate,
    redis::RedisConnect,
};
use qq_channel_warning::QqChannelGrpcService;
use tracing::instrument;

use super::error::{AnalyzeCookieError, AnalyzeCookieRResult};
use crate::router::AnalyzeCookieInside;

impl AnalyzeCookieInside {
    #[instrument(ret, skip(mongo, sql, redis_client, mob, qiniu))]
    pub async fn new_cookie(
        (mongo, sql, redis_client): (
            MongoDatabaseOperate,
            SqlDatabaseOperate,
            RedisConnect,
        ),
        (mob, qiniu, qq_channel): (
            PushManager,
            QiniuManager,
            QqChannelGrpcService,
        ),
        MapReject(cookie_req_info): MapReject<
            Json<Vec<NewCookieReq>>,
            AnalyzeCookieError,
        >,
    ) -> AnalyzeCookieRResult<()> {
        resp_try(async move {
            CeobeCookieLogic::new_cookie(
                mongo,
                sql,
                redis_client,
                mob,
                qq_channel,
                qiniu,
                cookie_req_info,
            )
            .await?;
            Ok(())
        })
        .await
    }
}
