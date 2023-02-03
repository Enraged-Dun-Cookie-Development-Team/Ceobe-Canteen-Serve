use ceobe_operate::ToCeobeOperation;
use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract, QueryCheckExtract,
};
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::video::{
        self, bv::query::Checked as BvQuery,
    },
};
use reqwest::Url;
use resp_result::{resp_try, rtry, RespResult};
use tracing::instrument;

use super::{
    error::{CeobeOperationVideoError, VideoRespResult},
    view::VideoItem,
    REQUEST_CLIENT,
};
use crate::router::CeobeOperationVideo;

type BvQueryCheck =
    QueryCheckExtract<video::bv::query::Checker, CeobeOperationVideoError>;

type UpdateVideoCheck = JsonCheckExtract<
    IntoIterChecker<Vec<video::Uncheck>, video::Checker, Vec<video::Checked>>,
    CeobeOperationVideoError,
>;

impl CeobeOperationVideo {
    #[instrument(ret)]
    pub async fn get_video_detail(
        CheckExtract(BvQuery { bv }): BvQueryCheck,
    ) -> VideoRespResult<String> {
        resp_try(async {
            let url = Url::parse_with_params(
                "https://api.bilibili.com/x/web-interface/view",
                &[("bvid", bv)],
            )?;

            let body = REQUEST_CLIENT.get(url).send().await?.bytes().await?;
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
                .ceobe_operation()
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
        rtry!(db.ceobe_operation().video().update_all(videos).await);
        RespResult::ok(())
    }
}
