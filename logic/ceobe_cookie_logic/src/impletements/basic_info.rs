use ceobe_cookie::{ToCeobe, ToCookie};
use db_ops_prelude::mongo_connection::MongoDatabaseOperate;
use tokio::task;

use super::CeobeCookieLogic;
use crate::{error::LogicResult, view::CookieNumberResp};

impl CeobeCookieLogic {
    pub async fn cookie_number(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<CookieNumberResp> {
        // 获取饼数量
        let cookie_count = task::spawn({
            let mongo = mongo.clone();
            async move {
                mongo.ceobe().cookie().analyze().get_cookie_count().await
            }
        });
        // 获取皮肤饼数量
        let costumes_count = task::spawn({
            let mongo = mongo.clone();
            let mut tags: Vec<&str> = Vec::<&str>::new();
            tags.push("皮肤");
            async move {
                mongo
                    .ceobe()
                    .cookie()
                    .analyze()
                    .get_tags_cookie_count(tags)
                    .await
            }
        });
        // 获取干员饼数量
        let operator_count = task::spawn({
            let mongo = mongo.clone();
            let mut tags: Vec<&str> = Vec::<&str>::new();
            tags.push("干员");
            async move {
                mongo
                    .ceobe()
                    .cookie()
                    .analyze()
                    .get_tags_cookie_count(tags)
                    .await
            }
        });
        // 获取活动饼数量
        let activity_count = task::spawn({
            let mongo = mongo.clone();
            let mut tags: Vec<&str> = Vec::<&str>::new();
            tags.push("SideStory");
            tags.push("故事集");
            async move {
                mongo
                    .ceobe()
                    .cookie()
                    .analyze()
                    .get_tags_cookie_count(tags)
                    .await
            }
        });
        // 获取ep饼数量
        let ep_count = task::spawn({
            let mongo = mongo.clone();
            let mut tags = Vec::<&str>::new();
            tags.push("EP");
            async move {
                mongo
                    .ceobe()
                    .cookie()
                    .analyze()
                    .get_tags_cookie_count(tags)
                    .await
            }
        });
        let cookie_count = cookie_count.await??;
        let costumes_count = costumes_count.await??;
        let operator_count = operator_count.await??;
        let activity_count = activity_count.await??;
        let ep_count = ep_count.await??;

        Ok(CookieNumberResp::builder()
            .total_count(cookie_count)
            .costumes_count(costumes_count)
            .operator_count(operator_count)
            .activity_count(activity_count)
            .ep_count(ep_count)
            .build())
    }
}
