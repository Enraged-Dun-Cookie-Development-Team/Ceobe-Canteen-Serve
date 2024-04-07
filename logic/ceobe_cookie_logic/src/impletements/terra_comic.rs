use std::collections::HashMap;

use persistence::{
    ceobe_cookie::{
        models::{
            analyze::models::{
                meta::TerraHistoricusExtra, CookieSimpleInfo,
                TerraComicEpisodeInfo,
            },
            terra_comic::models::ComicInfoWithoutCid,
        },
        terra_comic::OperateError as TerraComicOperateError,
        ToCeobe, ToCookie,
    },
    mongodb::MongoDatabaseOperate,
};
use tokio::task::{self, JoinHandle};

use super::CeobeCookieLogic;
use crate::{
    error::LogicResult,
    view::{TerraComicListResp, TerraEntryResp},
};

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

        let resp = comics_aggregate_list
            .into_iter()
            .filter(|comic| comics_set.contains_key(&comic.comic))
            .map(|comic| {
                TerraComicListResp {
                    time_count: comic.clone(),
                    info: comics_set.get(&comic.comic).unwrap().clone(),
                }
            })
            .collect();

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

    /// 泰拉记事社入口-获取最新小节漫画
    pub async fn newest_episode(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Option<TerraEntryResp>> {
        let episode: Option<CookieSimpleInfo> = mongo
            .ceobe()
            .cookie()
            .analyze()
            .get_newest_terra_comic_episode()
            .await?;

        if let Some(episode) = episode {
            // 这边在数据库查询时候已经保证这个字段存在
            let comic_id = serde_json::from_value::<TerraHistoricusExtra>(
                episode.meta.item.extra.into(),
            )?
            .comic;
            let mut cover_url: Option<String> = None;
            if let Some(images) = episode.images {
                cover_url =
                    images.first().map(|image| image.origin_url.clone());
            }
            let comic_info = mongo
                .ceobe()
                .cookie()
                .terra_comic()
                .find_comic_by_id(&comic_id)
                .await?;

            return Ok(Some(
                TerraEntryResp::builder()
                    .episode_short_title(episode.text)
                    .cover_url(cover_url)
                    .sub_title(
                        comic_info
                            .clone()
                            .map_or("".to_owned(), |comic_info| {
                                comic_info.subtitle
                            }),
                    )
                    .title(
                        comic_info.map_or("".to_owned(), |comic_info| {
                            comic_info.title
                        }),
                    )
                    // 泰拉记事社漫画数据源一定有platform这个时间字段
                    .updated_time(episode.meta.timestamp.platform.unwrap())
                    .build(),
            ));
        }
        Ok(None)
    }
}
