use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract, QueryCheckExtract,
};
use persistence::{
    ceobe_operate::{
        models::video::{self, bv::query::Checked as BvQuery},
        ToCeobe, ToCeobeOperation,
    },
    mysql::SqlDatabaseOperate,
};
use request_clients::bili_client::QueryBiliVideo;
use resp_result::{resp_try, rtry, RespResult};
use tracing::{event, instrument, Level};

use super::{
    error::{CeobeOperationVideoError, VideoRespResult},
    view::VideoItem,
};
use crate::router::CeobeOperationVideo;

type BvQueryCheck =
    QueryCheckExtract<video::bv::query::Checker, CeobeOperationVideoError>;

type UpdateVideoCheck = JsonCheckExtract<
    IntoIterChecker<Vec<video::Uncheck>, video::Checker, Vec<video::Checked>>,
    CeobeOperationVideoError,
>;

impl CeobeOperationVideo {
    #[instrument(skip(query))]
    pub async fn get_video_detail(
        CheckExtract(BvQuery { bv }): BvQueryCheck, query: QueryBiliVideo,
    ) -> VideoRespResult<String> {
        resp_try(async {
            let body = query.fetch(bv).await??;
            event!(Level::INFO, response.len = body.len());
            Ok(String::from_utf8(body.to_vec())?)
        })
        .await
    }

    #[instrument(ret, skip(database))]
    pub async fn list_all(
        database: SqlDatabaseOperate,
    ) -> VideoRespResult<Vec<VideoItem>> {
        resp_try(async {
            Ok(database
                .ceobe()
                .operation()
                .video()
                .find_all_not_delete()
                .await?
                .into_iter()
                .map(Into::into)
                .collect())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn update_list(
        db: SqlDatabaseOperate, CheckExtract(videos): UpdateVideoCheck,
    ) -> VideoRespResult<()> {
        rtry!(db.ceobe().operation().video().update_all(videos).await);
        RespResult::ok(())
    }
}
