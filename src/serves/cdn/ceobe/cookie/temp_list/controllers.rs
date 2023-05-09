use axum::extract::Query;
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic,
    view::{CookieListReq, CookieTempListResp},
};
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use super::error::{CeobeCookieRResult, CeobeCookieTempListError};
use crate::router::CdnCookieTempFrontend;

impl CdnCookieTempFrontend {
    #[instrument(ret, skip(db, mongo))]
    pub async fn cookie_list(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        MapReject(cookie_req_info): MapReject<
            Query<CookieListReq>,
            CeobeCookieTempListError,
        >,
    ) -> CeobeCookieRResult<CookieTempListResp> {
        Ok(rtry!(
            CeobeCookieLogic::get_temp_cookies_by_pagenation(
                db,
                mongo,
                cookie_req_info
            )
            .await
        ))
        .into()
    }
}
