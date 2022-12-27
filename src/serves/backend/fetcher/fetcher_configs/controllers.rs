use orm_migrate::sql_connection::SqlConnect;
use redis_connection::RedisConnect;
use resp_result::{rtry, resp_try};
use tracing::instrument;
use crate::{router::FetcherConfigControllers, serves::backend::fetcher::fetcher_configs::view::MaxLiveNumberResp};

use super::error::FetcherConfigRResult;

impl FetcherConfigControllers {
    // 获取到目前为止最大存活蹲饼器数量
    #[instrument(ret, skip(client))]
    pub async fn get_fetcher_max_live_number(
        mut client: RedisConnect,
    ) -> FetcherConfigRResult<MaxLiveNumberResp> {
        resp_try(async {
            let number = fetcher_logic::implement::get_cookie_fetcher_max_live_number(&mut client).await?;
            let resp = MaxLiveNumberResp {fetcher_live_number: number};

            Ok(resp)
        })
        .await
    }
    
    // 上传蹲饼器配置
    #[instrument(ret, skip(db))]
    pub async fn upload_fetchers_configs(
        db: SqlConnect,
    ) -> FetcherConfigRResult<()> {
        Ok(()).into()
    }
}
