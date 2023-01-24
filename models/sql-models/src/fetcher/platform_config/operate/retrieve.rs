use std::ops::Deref;

use page_size::{database::WithPagination, request::Paginator};
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QuerySelect};
use smallvec::SmallVec;
use sql_connection::database_traits::{
    database_operates::NoConnect, get_connect::GetDatabaseConnect,
};
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{OperateResult, Platform};
use crate::fetcher::platform_config::{
    models::model_platform_config,
    operate::retrieve::model_platform_config::{
        PlatformBasicInfo, PlatformType,
    },
};

impl Platform<'_, NoConnect> {
    #[instrument(skip(db))]
    /// 分页获取全部平台列表
    pub async fn find_all_with_paginator(
        db: &impl ConnectionTrait, page_size: Paginator,
    ) -> OperateResult<Vec<model_platform_config::Model>> {
        info!(
            platformList.page.num = page_size.page.deref(),
            platformList.page.size = page_size.size.deref()
        );
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
}

impl<'c, C> Platform<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect<'c>: ConnectionTrait,
{
    #[instrument(skip_all)]
    /// 获取全部平台type_id列表
    pub async fn find_all(&'c self) -> OperateResult<Vec<String>> {
        let db = self.get_connect();
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

    #[instrument(skip_all)]
    /// 获取全部平台基础信息列表
    pub async fn find_all_basic_info(
        &'c self,
    ) -> OperateResult<Vec<PlatformBasicInfo>> {
        let db = self.get_connect();
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

    #[instrument(skip_all, ret)]
    /// 获取平台总数
    pub async fn count_all(&'c self) -> OperateResult<u64> {
        let db = self.get_connect();
        model_platform_config::Entity::find()
            .count(db)
            .await
            .map_err(Into::into)
    }
}
