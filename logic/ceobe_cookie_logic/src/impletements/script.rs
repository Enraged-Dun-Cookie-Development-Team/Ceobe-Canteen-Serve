use std::collections::HashMap;

use ceobe_qiniu_upload::QiniuManager;
use persistence::{redis::RedisConnect, operate::GetMutDatabaseConnect};
use qiniu_service::QiniuService;
use qq_channel_warning::QqChannelGrpcService;
use redis::AsyncCommands;
use redis_global::redis_key::cookie_list::CookieListKey;

use crate::error::LogicResult;

use super::CeobeCookieLogic;

impl CeobeCookieLogic {
    // 缓慢同步redis的combid数据到七牛云
    pub async fn synchronous_qiniu_from_redis(
        redis_client: &mut RedisConnect, 
        mut qq_channel: QqChannelGrpcService, qiniu: QiniuManager,
    ) -> LogicResult<()> {
        let redis = redis_client.mut_connect();
        let comb_ids: HashMap<String, String> = redis.hgetall(CookieListKey::NEWEST_COOKIES).await?;
        for (comb_id, cookie_id) in comb_ids.into_iter() {
            QiniuService::upload_newest_cookie_id_use_script(
                qiniu.clone(),
                cookie_id,
                &mut qq_channel,
                comb_id.to_owned(),
            ).await;
        }

        Ok(())
    }
}