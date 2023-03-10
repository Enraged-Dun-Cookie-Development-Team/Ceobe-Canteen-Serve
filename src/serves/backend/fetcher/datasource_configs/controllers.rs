use abstract_database::fetcher::ToFetcher;
use axum::{extract::{Query, multipart::MultipartRejection, Multipart}, Json};
use ceobe_qiniu_upload::QiniuUploader;
use checker::CheckExtract;
use fetcher::{datasource_config::ToDatasource, platform_config::ToPlatform};
use fetcher_logic::{
    implements::FetcherConfigLogic,
    view::{
        DatasourceList, DatasourceListFilterCondReq, DatasourceWithNameResp,
        OneIdReq, PlatformAndDatasourceArrayResp, PlatformFilterReq,
    },
};
use futures::future;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use page_size::response::{GenerateListWithPageInfo, ListWithPageInfo};
use qiniu_cdn_upload::upload;
use resp_result::{resp_try, rtry, MapReject};
use scheduler_notifier::SchedulerNotifier;
use tracing::instrument;

use super::{
    error::{DatasourceConfigError, DatasourceConfigRResult},
    FetcherDatasourceCheck, PageSizePretreatment,
};
use crate::{router::FetcherConfigControllers, serves::backend::fetcher::datasource_configs::{view::AvatarId, error::FieldNotExist, DataSourceAvatarPayload}};

impl FetcherConfigControllers {
    /// 获取平台与数据源类型列表
    #[instrument(ret, skip(db))]
    pub async fn get_platform_and_datasource_list(
        db: SqlDatabaseOperate,
    ) -> DatasourceConfigRResult<PlatformAndDatasourceArrayResp> {
        resp_try(async {
            // 异步获取
            let (platform_list, datasource_list) = future::join(
                // 获取数据源数量
                db.fetcher().platform().find_all(),
                // 获取平台列表
                db.fetcher().datasource().find_all_type(),
            )
            .await;

            let resp = PlatformAndDatasourceArrayResp {
                platform_list: platform_list?,
                datasource_list: datasource_list?,
            };

            Ok(resp)
        })
        .await
    }

    /// 获取数据源列表
    #[instrument(ret, skip(db))]
    pub async fn get_datasource_list(
        db: SqlDatabaseOperate,
        CheckExtract(page_size): PageSizePretreatment,
        MapReject(filter_cond): MapReject<
            Query<DatasourceListFilterCondReq>,
            DatasourceConfigError,
        >,
    ) -> DatasourceConfigRResult<ListWithPageInfo<DatasourceList>> {
        resp_try(async {
            // 获取数据源列表
            // 获取数据源数量
            // 异步获取
            let (datasource_list, count) = future::join(
                db.fetcher().datasource().find_all_with_paginator(
                    page_size,
                    filter_cond.platform.clone(),
                    filter_cond.datasource.clone(),
                ),
                db.fetcher()
                    .datasource()
                    .count(filter_cond.platform, filter_cond.datasource),
            )
            .await;

            let datasource_list = datasource_list?
                .into_iter()
                .map(Into::into)
                .collect::<Vec<DatasourceList>>();
            let resp = datasource_list.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    /// 上传数据源配置
    #[instrument(ret, skip(db))]
    // #[axum::debug_handler]
    pub async fn create_datasource_config(
        db: SqlDatabaseOperate,
        CheckExtract(datasource_config): FetcherDatasourceCheck,
    ) -> DatasourceConfigRResult<()> {
        resp_try(async {
            FetcherConfigLogic::create_datasource_config(
                db,
                datasource_config,
            )
            .await?;
            Ok(())
        })
        .await
    }

    // 更新数据源配置
    #[instrument(ret, skip(db))]
    pub async fn update_datasource_config(
        db: SqlDatabaseOperate,
        CheckExtract(datasource_config): FetcherDatasourceCheck,
    ) -> DatasourceConfigRResult<()> {
        rtry!(db.fetcher().datasource().update(datasource_config).await);
        Ok(()).into()
    }

    // 删除数据源配置
    #[instrument(ret, skip(db, notifier))]
    pub async fn delete_datasource_config(
        db: SqlDatabaseOperate, notifier: SchedulerNotifier,
        MapReject(datasource): MapReject<
            Json<OneIdReq>,
            DatasourceConfigError,
        >,
    ) -> DatasourceConfigRResult<()> {
        rtry!(
            FetcherConfigLogic::delete_datasource_by_id(
                &notifier,
                db,
                datasource.id
            )
            .await
        );
        Ok(()).into()
    }

    /// 获取数据源配置全列表(包含id、名字、config)
    #[instrument(ret, skip(db))]
    pub async fn get_datasource_name_list(
        db: SqlDatabaseOperate,
        MapReject(filter): MapReject<
            Query<PlatformFilterReq>,
            DatasourceConfigError,
        >,
    ) -> DatasourceConfigRResult<Vec<DatasourceWithNameResp>> {
        resp_try(async {
            let list = db
                .fetcher()
                .datasource()
                .find_by_platform(&filter.type_id)
                .await?;
            let resp = list.into_iter().map(Into::into).collect();
            Ok(resp)
        })
        .await
    }

    /// 上传数据源头像
    #[instrument(ret, skip(uploader))]
    pub async fn upload_avatar(
        uploader: QiniuUploader,
        multipart: Result<Multipart, MultipartRejection>,
    ) -> DatasourceConfigRResult<AvatarId> {
        resp_result::resp_try(async move {
            let mut multipart = multipart?;
            let field =
                multipart.next_field().await?.ok_or(FieldNotExist)?;

            let resp =
                upload(&uploader, field, DataSourceAvatarPayload::new())
                    .await
                    .map(|resp| AvatarId::from_resp(resp, &uploader))?;

            Ok(resp)
        })
        .await
    }
}
