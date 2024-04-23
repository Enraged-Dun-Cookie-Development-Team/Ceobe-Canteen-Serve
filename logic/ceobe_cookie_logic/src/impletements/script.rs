use ceobe_qiniu_upload::QiniuManager;
use persistence::{operate::GetMutDatabaseConnect, redis::RedisConnect};
use qiniu_service::QiniuService;
use qq_channel_warning::QqChannelGrpcService;
use redis_global::{
    redis_key::cookie_list::NewestCookies
    , RedisTypeBind,
};

use crate::error::LogicResult;

use super::CeobeCookieLogic;

impl CeobeCookieLogic {
    #[deprecated]
    #[allow(deprecated)]
    // 缓慢同步redis的combid数据到七牛云
    pub async fn synchronous_qiniu_from_redis(
        redis_client: &mut RedisConnect,
        mut qq_channel: QqChannelGrpcService, qiniu: QiniuManager,
    ) -> LogicResult<()> {
        let redis = redis_client.mut_connect();
        let comb_ids =
            NewestCookies.bind(redis).all::<String>().await?;
        for (comb_id, cookie_id) in comb_ids.into_iter() {
            QiniuService::upload_newest_cookie_id_use_script(
                qiniu.clone(),
                cookie_id,
                &mut qq_channel,
                comb_id.to_owned(),
            )
            .await;
        }

        Ok(())
    }
}
