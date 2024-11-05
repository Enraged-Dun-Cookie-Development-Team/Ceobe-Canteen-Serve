use axum::extract::Query;
use axum_resp_result::{rtry, MapReject};
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic,
    view::{CookieListResp, SearchListReq},
};
use persistence::{mongodb::MongoDatabaseOperate, mysql::SqlDatabaseOperate};
use tracing::instrument;

use super::error::{CeobeCookieRResult, CeobeCookieSearchError};
use crate::router::CookieSearchFrontend;

impl CookieSearchFrontend {
    /// 搜索列表返回
    #[instrument(ret, skip(sql, mongo))]
    pub async fn search_list(
        sql: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        MapReject(search_info): MapReject<
            Query<SearchListReq>,
            CeobeCookieSearchError,
        >,
    ) -> CeobeCookieRResult<CookieListResp> {
        Ok(rtry!(
            CeobeCookieLogic::search_list(sql, mongo, search_info).await
        ))
        .into()
    }
}
