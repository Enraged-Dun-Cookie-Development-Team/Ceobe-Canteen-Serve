use axum::extract::Query;
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic,
    view::{CookieListReq, CookieListResp},
};
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use redis_connection::RedisConnect;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use super::error::{CeobeCookieMainListError, CeobeCookieRResult};
use crate::router::CdnCookieMainListFrontend;

impl CdnCookieMainListFrontend {
    #[instrument(ret, skip(db, redis_client, mongo))]
    pub async fn cookie_list(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        redis_client: RedisConnect,
        MapReject(cookie_req_info): MapReject<
            Query<CookieListReq>,
            CeobeCookieMainListError,
        >,
    ) -> CeobeCookieRResult<CookieListResp> {
        Ok(rtry!(
            CeobeCookieLogic::cookie_list(
                db,
                mongo,
                redis_client,
                cookie_req_info
            )
            .await
        ))
        .into()
    }
}
