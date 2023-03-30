use axum::{Json, debug_handler, extract::State};
use ceobe_cookie_logic::{view::NewCookieReq, impletements::CeobeCookieLogic};
use ceobe_qiniu_upload::QiniuManager;
use general_request_client::client::RequestClient;
use mob_push_server::PushManager;
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{MapReject, resp_try};
use tracing::instrument;

use crate::router::AnalyzeCookieInside;

use super::error::{AnalyzeCookieRResult, AnalyzeCookieError};



impl AnalyzeCookieInside {
    #[instrument(ret, skip(mongo, sql, request_client, mob, qiniu))]
    pub async fn new_cookie(
        mongo: MongoDatabaseOperate,
        sql: SqlDatabaseOperate,
        State(request_client): State<RequestClient>,
        mob: PushManager,
        qiniu: QiniuManager,
        MapReject(cookie_req_info): MapReject<
            Json<NewCookieReq>,
            AnalyzeCookieError,
        >,
    ) -> AnalyzeCookieRResult<()> {
        resp_try(async move {
            CeobeCookieLogic::new_cookie(mongo, sql, request_client, mob, qiniu, cookie_req_info).await?;
            Ok(())
        })
        .await
    }
}