use axum::{extract::Query, Json};
use fetcher_logic::view::{
    BackFetcherConfig, MaxLiveNumberResp, PlatformFilterReq,
};
use orm_migrate::sql_connection::SqlConnect;
use redis_connection::RedisConnect;
use resp_result::{resp_try, rtry, MapReject};
use tracing::instrument;

use super::error::{FetcherConfigError, FetcherConfigRResult};
use crate::router::FetcherConfigControllers;

impl FetcherConfigControllers {
    /// 获取到目前为止最大存活蹲饼器数量
    #[instrument(ret, skip(client))]
    pub async fn get_fetcher_max_live_number(
        mut client: RedisConnect,
    ) -> FetcherConfigRResult<MaxLiveNumberResp> {
        resp_try(async {
            let number =
                fetcher_logic::implement::get_cookie_fetcher_max_live_number(
                    &mut client,
                )
                .await?;
            let resp = MaxLiveNumberResp {
                fetcher_live_number: number,
            };

            Ok(resp)
        })
        .await
    }

    /// 上传蹲饼器配置
    #[instrument(ret, skip(db, configs))]
    pub async fn upload_fetchers_configs(
        db: SqlConnect,
        MapReject(configs): MapReject<
            Json<Vec<BackFetcherConfig>>,
            FetcherConfigError,
        >,
    ) -> FetcherConfigRResult<()> {
        rtry!(
            fetcher_logic::implement::upload_cookie_fetcher_configs(
                &db, configs
            )
            .await
        );
        Ok(()).into()
    }

    /// 根据平台获取蹲饼器配置
    #[instrument(skip(db))]
    pub async fn get_fetchers_configs(
        db: SqlConnect,
        MapReject(PlatformFilterReq{ type_id }): MapReject<
            Query<PlatformFilterReq>,
            FetcherConfigError,
        >,
    ) -> FetcherConfigRResult<Vec<BackFetcherConfig>> {
        Ok(rtry!(
            fetcher_logic::implement::get_cookie_fetcher_configs(
                &db,
                type_id
            )
            .await
        ))
        .into()
    }
}
