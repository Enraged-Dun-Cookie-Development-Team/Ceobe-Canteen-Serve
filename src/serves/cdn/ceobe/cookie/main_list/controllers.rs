use axum::extract::Query;
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic,
    view::{CookieListReq, CookieTempListResp, CookieListResp},
};
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use super::error::{CeobeCookieRResult, CeobeCookieMainListError};
use crate::router::CdnCookieMainListFrontend;



impl CdnCookieMainListFrontend {
    #[instrument(ret, skip(db, mongo))]
    pub async fn cookie_list(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        MapReject(cookie_req_info): MapReject<
            Query<CookieListReq>,
            CeobeCookieMainListError,
        >,
    ) -> CeobeCookieRResult<CookieListResp> {
        Ok(rtry!(
            CeobeCookieLogic::cookie_list(
                db,
                mongo,
                cookie_req_info
            )
            .await
        ))
        .into()
    }
}