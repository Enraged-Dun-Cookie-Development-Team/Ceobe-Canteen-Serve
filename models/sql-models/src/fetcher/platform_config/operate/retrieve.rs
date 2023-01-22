use std::ops::Deref;

use page_size::{database::WithPagination, request::Paginator};
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QuerySelect};
use smallvec::SmallVec;
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{FetcherPlatformConfigSqlOperate, OperateResult};
use crate::fetcher::platform_config::{
    models::model_platform_config,
    operate::retrieve::model_platform_config::{
        PlatformBasicInfo, PlatformType,
    },
};

impl FetcherPlatformConfigSqlOperate {
    #[instrument(skip(db))]
    /// 分页获取全部平台列表
    pub async fn find_all_with_paginator<'db, D>(
        db: &'db D, page_size: Paginator,
    ) -> OperateResult<Vec<model_platform_config::Model>>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            platformList.page.num = page_size.page.deref(),
            platformList.page.size = page_size.size.deref()
        );
        let db = db.get_connect();
        Ok(model_platform_config::Entity::find()
            .with_pagination(page_size)
            .all(db)
            .await?).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    let list = list.iter().map(|platform|(&platform.type_id)).collect::<SmallVec<[_;4]>>();
                    info!(platformList.len = list.len(),  platformList.platform.pType = ?list );
                });
            })
    }

    #[instrument(skip(db))]
    /// 获取全部平台type_id列表
    pub async fn find_all<'db, D>(db: &'db D) -> OperateResult<Vec<String>>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect();
        Ok(model_platform_config::Entity::find()
            .select_only()
            .column(model_platform_config::Column::TypeId)
            .into_model::<PlatformType>()
            .all(db)
            .await?
            .into_iter()
            .map(|item| item.type_id)
            .collect::<Vec<String>>()
            ).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    info!(platformList.len = list.len(),  platformList.platform.pType = ?list );
                });
            })
    }

    #[instrument(skip(db))]
    /// 获取全部平台基础信息列表
    pub async fn find_all_basic_info<'db, D>(
        db: &'db D,
    ) -> OperateResult<Vec<PlatformBasicInfo>>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect();
        Ok(model_platform_config::Entity::find()
            .into_model::<PlatformBasicInfo>()
            .all(db)
            .await?).tap_ok(|list| {
                Span::current()
                    .in_scope(||{
                    let list = list.iter().map(|platform|(&platform.platform_name)).collect::<SmallVec<[_;4]>>();
                    info!(platformList.len = list.len(),  platformList.platform.name = ?list );
                });
            })
    }

    #[instrument(skip(db), ret)]
    /// 获取平台总数
    pub async fn count_all<'db, D>(db: &'db D) -> OperateResult<u64>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect();
        model_platform_config::Entity::find()
            .count(db)
            .await
            .map_err(Into::into)
    }
}
