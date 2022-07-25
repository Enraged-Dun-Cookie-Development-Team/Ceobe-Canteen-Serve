use axum_prehandle::{
    prefabs::{json::JsonPayload, query::QueryParams},
    PreHandling, PreRespHandling,
};
use checker::prefabs::collect_checkers::iter_checkers::IntoIterChecker;
use orm_migrate::sql_models::ceobe_operation::video::{
    checkers::{
        bv_arg_checker::{BvQuery, BvQueryChecker, BvQueryUncheck},
        video_data::{
            CeobeOpVideoChecker, CeobeOpVideoUncheck, CeobeOpVideo,
        },
    },
    operate::CeobeOperationVideoSqlOperate,
};
use reqwest::Url;
use resp_result::RespResult;

use super::{
    error::{CeobeOperationVideoError, VideoRespResult},
    view::VideoItem,
    VideoAuthentication, REQUEST_CLIENT,
};
use crate::{
    router::CeobeOperationVideo, utils::data_checker::PreLiteChecker,
};

type BvQueryCheck = PreLiteChecker<
    QueryParams<BvQueryUncheck>,
    BvQueryChecker,
    CeobeOperationVideoError,
>;

type UpdateVideoCheck = PreLiteChecker<
    JsonPayload<Vec<CeobeOpVideoUncheck>>,
    IntoIterChecker<
        Vec<CeobeOpVideoUncheck>,
        CeobeOpVideoChecker,
        Vec<CeobeOpVideo>,
    >,
    CeobeOperationVideoError,
>;

impl CeobeOperationVideo {
    pub async fn get_video_detail(
        _: VideoAuthentication,
        PreHandling(BvQuery { bv }): PreRespHandling<BvQueryCheck>,
    ) -> VideoRespResult<String> {
        let url = Url::parse_with_params(
            "https://api.bilibili.com/x/web-interface/view",
            &[("bvid", bv)],
        )?;

        let body = REQUEST_CLIENT.get(url).send().await?.bytes().await?;
        RespResult::ok(String::from_utf8(body.to_vec())?)
    }

    pub async fn list_all(
        _: VideoAuthentication,
    ) -> VideoRespResult<Vec<VideoItem>> {
        RespResult::ok(
            CeobeOperationVideoSqlOperate::find_all_not_delete()
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
        )
    }

    pub async fn update_list(
        _: VideoAuthentication,
        PreHandling(videos): PreRespHandling<UpdateVideoCheck>,
    ) -> VideoRespResult<()> {
        CeobeOperationVideoSqlOperate::update_all(videos).await?;
        RespResult::ok(())
    }
}
