use std::sync::Arc;

use axum::{Extension, Json};
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
use resp_result::{resp_try, MapReject};
use tokio::sync::Mutex;
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
        Extension(mutex): Extension<Arc<Mutex<()>>>,
        MapReject(cookie_req_info): MapReject<
            Json<Vec<NewCookieReq>>,
            AnalyzeCookieError,
        >,
    ) -> AnalyzeCookieRResult<()> {
        resp_try(async move {
            // 添加公平锁，避免七牛云上传过程顺序错误
            let mutex_guard = mutex.lock().await;
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
            drop(mutex_guard);
            Ok(())
        })
        .await
    }
}
