use std::collections::HashMap;

use ceobe_cookie::{
    terra_comic::OperateError as TerraComicOperateError, ToCeobe, ToCookie,
};
use db_ops_prelude::{
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::cookie::{
        analyze::models::TerraComicEpisodeInfo,
        terra_comic::models::ComicInfoWithoutCid,
    },
};
use tokio::task::{self, JoinHandle};

use super::CeobeCookieLogic;
use crate::{error::LogicResult, view::TerraComicListResp};

impl CeobeCookieLogic {
    /// 获取漫画列表
    pub async fn comic_list(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Vec<TerraComicListResp>> {
        let comics_set: JoinHandle<Result<_, TerraComicOperateError>> =
            task::spawn({
                let mongo = mongo.clone();
                async move {
                    let comics_list = mongo
                        .ceobe()
                        .cookie()
                        .terra_comic()
                        .find_all_comic()
                        .await?;
                    Ok(comics_list
                        .into_iter()
                        .map(|info| (info.cid.clone(), info.into()))
                        .collect::<HashMap<String, ComicInfoWithoutCid>>())
                }
            });
        let comics_aggregate_list = task::spawn({
            async move {
                mongo
                    .ceobe()
                    .cookie()
                    .analyze()
                    .get_each_terra_comic_count()
                    .await
            }
        });
        let comics_set = comics_set.await??;
        let comics_aggregate_list = comics_aggregate_list.await??;

        let mut resp = Vec::<TerraComicListResp>::new();
        for comic in comics_aggregate_list {
            if !comics_set.contains_key(&comic.comic) {
                continue;
            }
            resp.push(TerraComicListResp {
                time_count: comic.clone(),
                info: comics_set.get(&comic.comic).unwrap().clone(),
            });
        }

        Ok(resp)
    }

    /// 获取一个漫画下集列表信息
    pub async fn comic_episode_list(
        mongo: MongoDatabaseOperate, comic_id: String,
    ) -> LogicResult<Vec<TerraComicEpisodeInfo>> {
        Ok(mongo
            .ceobe()
            .cookie()
            .analyze()
            .get_terra_comic_episode_list(comic_id)
            .await?)
    }
}
