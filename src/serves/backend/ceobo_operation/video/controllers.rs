use axum_prehandle::{
    prefabs::query::QueryParams, PreHandling, PreRespHandling,
};
use orm_migrate::sql_models::ceobe_operation::video::checkers::{
    bv_arg_checker::{BvQuery, BvQueryChecker, BvQueryUncheck},
};
use reqwest::Url;
use resp_result::RespResult;

use super::{
    error::{CeobeOperationVideoError, VideoRespResult},
    REQUEST_CLIENT,
};
use crate::{
    router::CeoboOperationVideo, utils::data_checker::PreLiteChecker,
};

type BvQueryCheck = PreLiteChecker<
    QueryParams<BvQueryUncheck>,
    BvQueryChecker,
    CeobeOperationVideoError,
>;

impl CeoboOperationVideo {
    pub async fn get_video_detail(
        PreHandling(BvQuery { bv }): PreRespHandling<BvQueryCheck>,
    ) -> VideoRespResult<String> {
        let url = Url::parse_with_params(
            "https://api.bilibili.com/x/web-interface/view",
            &[("aid", bv.to_av().to_string().as_str())],
        )?;

        let body = REQUEST_CLIENT.get(url).send().await?.bytes().await?;
        RespResult::ok(String::from_utf8(body.to_vec())?)
    }
}
