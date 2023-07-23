use persistence::{
    ceobe_cookie::{ToCeobe, ToCookie},
    mongodb::MongoDatabaseOperate
};
use tokio::task;

use super::CeobeCookieLogic;
use crate::{
    error::{LogicError, LogicResult},
    view::CookieNumberResp,
};

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
        let tags_list: [&[&str]; 4] =
            [&["皮肤"], &["干员"], &["故事集", "SideStory"], &["EP"]];
        let count_list =
            futures::future::join_all(tags_list.into_iter().map(|tags| {
                task::spawn({
                    let mongo = mongo.clone();
                    async move {
                        mongo
                            .ceobe()
                            .cookie()
                            .analyze()
                            .get_cookie_count_by_tags(tags)
                            .await
                    }
                })
            }))
            .await;

        let count_list = count_list.into_iter().try_fold::<_, _, Result<
            Vec<u64>,
            LogicError,
        >>(
            Vec::<u64>::new(),
            |mut vec, x| {
                vec.push(x??);
                Ok(vec)
            },
        )?;
        let &[skin_count, operator_count, activity_count, ep_count] =
            count_list.as_slice()
        else {
            unreachable!()
        };

        let cookie_count = cookie_count.await??;

        Ok(CookieNumberResp::builder()
            .total_count(cookie_count)
            .skin_count(skin_count)
            .operator_count(operator_count)
            .activity_count(activity_count)
            .ep_count(ep_count)
            .build())
    }
}
