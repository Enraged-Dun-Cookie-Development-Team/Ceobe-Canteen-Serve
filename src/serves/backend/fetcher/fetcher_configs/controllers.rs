use axum::{extract::Query, Json};
use fetcher_logic::{
    implements::{FetcherConfigLogic, SuperLogic, DetailConfig},
    view::{BackEndFetcherConfig, MaxLiveNumberResp, PlatformFilterReq},
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
            let number = FetcherConfigLogic
                .sub_logic::<DetailConfig>()
                .get_max_live_number(&mut client)
                .await?;
            let resp = MaxLiveNumberResp {
                fetcher_live_number: number,
            };

            Ok(resp)
        })
        .await
    }

    /// 上传蹲饼器配置
    // #[instrument(ret, skip(db, configs))]
    pub async fn upload_fetchers_configs(
        db: SqlConnect,
        MapReject(configs): MapReject<
            Json<Vec<BackEndFetcherConfig>>,
            FetcherConfigError,
        >,
    ) -> FetcherConfigRResult<()> {
        resp_try(async move {
            FetcherConfigLogic
            .sub_logic::<DetailConfig>()
                .upload_multi(&db, configs)
                .await?;
            Ok(())
        })
        .await
    }

    /// 根据平台获取蹲饼器配置
    #[instrument(skip(db))]
    pub async fn get_fetchers_configs(
        db: SqlConnect,
        MapReject(PlatformFilterReq { type_id }): MapReject<
            Query<PlatformFilterReq>,
            FetcherConfigError,
        >,
    ) -> FetcherConfigRResult<Vec<BackEndFetcherConfig>> {
        Ok(rtry!(
            FetcherConfigLogic
            .sub_logic::<DetailConfig>()
                .get_by_platform(&db, &type_id)
                .await
        ))
        .into()
    }
}
