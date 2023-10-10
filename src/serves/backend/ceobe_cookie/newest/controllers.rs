use ceobe_cookie_logic::impletements::CeobeCookieLogic;
use ceobe_qiniu_upload::QiniuManager;
use persistence::redis::RedisConnect;
use qq_channel_warning::QqChannelGrpcService;
use resp_result::resp_try;
use tracing::instrument;

use super::error::CookieNewestRResult;
use crate::router::CeobeCookieNewestBackend;

impl CeobeCookieNewestBackend {
    #[instrument(ret, skip(redis_client, qiniu))]
    pub async fn synchronous_qiniu_from_redis(
        mut redis_client: RedisConnect,
        (qiniu, qq_channel): (QiniuManager, QqChannelGrpcService),
    ) -> CookieNewestRResult<()> {
        resp_try(async move {
            CeobeCookieLogic::synchronous_qiniu_from_redis(
                &mut redis_client,
                qq_channel,
                qiniu,
            )
            .await?;
            Ok(())
        })
        .await
    }
}
