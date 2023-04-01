use std::sync::Arc;

use axum::{Extension, Json};
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic, view::NewCookieReq,
};
use ceobe_qiniu_upload::QiniuManager;
use mob_push_server::PushManager;
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{resp_try, MapReject};
use tokio::sync::Mutex;
use tracing::instrument;

use super::error::{AnalyzeCookieError, AnalyzeCookieRResult};
use crate::router::AnalyzeCookieInside;

impl AnalyzeCookieInside {
    #[instrument(ret, skip(mongo, sql, mob, qiniu))]
    pub async fn new_cookie(
        mongo: MongoDatabaseOperate, sql: SqlDatabaseOperate,
        mob: PushManager, qiniu: QiniuManager,
        Extension(mutex): Extension<Arc<Mutex<()>>>,
        MapReject(cookie_req_info): MapReject<
            Json<NewCookieReq>,
            AnalyzeCookieError,
        >,
    ) -> AnalyzeCookieRResult<()> {
        resp_try(async move {
            // 添加公平锁，避免七牛云上传过程顺序错误
            let mutex_guard = mutex.lock().await;
            CeobeCookieLogic::new_cookie(
                mongo,
                sql,
                mob,
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
