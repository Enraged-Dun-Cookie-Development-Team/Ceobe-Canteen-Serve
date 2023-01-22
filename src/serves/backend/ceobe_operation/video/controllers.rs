use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract, QueryCheckExtract,
};
use database_traits::database_operates::sub_operate::SuperOperate;
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::{
        video::{
            checkers::{
                bv_arg_checker::{BvQuery, BvQueryChecker},
                video_data::{
                    CeobeOpVideo, CeobeOpVideoChecker, CeobeOpVideoUncheck,
                },
            },
            operate::VideoOperate,
        },
        SqlCeobeOperation,
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
        mut database: SqlDatabaseOperate,
    ) -> VideoRespResult<Vec<VideoItem>> {
        resp_try(async {
            Ok(database
                .child::<SqlCeobeOperation<_>>()
                .child::<VideoOperate<_>>()
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
        mut db: SqlDatabaseOperate, CheckExtract(videos): UpdateVideoCheck,
    ) -> VideoRespResult<()> {
        rtry!(
            db.child::<SqlCeobeOperation<_>>()
                .child::<VideoOperate<_>>()
                .update_all(videos)
                .await
        );
        RespResult::ok(())
    }
}
