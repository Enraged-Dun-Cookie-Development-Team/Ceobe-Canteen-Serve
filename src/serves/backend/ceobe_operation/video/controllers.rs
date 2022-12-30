use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract, QueryCheckExtract,
};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::ceobe_operation::video::{
        checkers::{
            bv_arg_checker::{BvQuery, BvQueryChecker},
            video_data::{
                CeobeOpVideo, CeobeOpVideoChecker, CeobeOpVideoUncheck,
            },
        },
        operate::CeobeOperationVideoSqlOperate,
    },
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
    QueryCheckExtract<BvQueryChecker, CeobeOperationVideoError>;

type UpdateVideoCheck = JsonCheckExtract<
    IntoIterChecker<
        Vec<CeobeOpVideoUncheck>,
        CeobeOpVideoChecker,
        Vec<CeobeOpVideo>,
    >,
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

    #[instrument(ret, skip(db))]
    pub async fn list_all(db: SqlConnect) -> VideoRespResult<Vec<VideoItem>> {
        resp_try(async {
            Ok(CeobeOperationVideoSqlOperate::find_all_not_delete(&db)
                .await?
                .into_iter()
                .map(Into::into)
                .collect())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn update_list(
        db: SqlConnect, CheckExtract(videos): UpdateVideoCheck,
    ) -> VideoRespResult<()> {
        rtry!(CeobeOperationVideoSqlOperate::update_all(&db, videos).await);
        RespResult::ok(())
    }
}
