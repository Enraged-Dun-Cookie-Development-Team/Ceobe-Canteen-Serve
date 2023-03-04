use std::ops::Deref;

use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{
        ConnectionTrait, EntityTrait, PaginatorTrait, QuerySelect,
        StreamTrait,
    },
    smallvec::SmallVec,
    sql_models::fetcher::platform_config::models::model_platform_config::{
        self, Entity, PlatformBasicInfo, PlatformType,
    },
};
use page_size::{database::WithPagination, request::Paginator};
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{OperateResult, PlatformOperate};

impl<'c, C> PlatformOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    /// 分页获取全部平台列表
    pub async fn find_all_with_paginator(
        &self, page_size: Paginator,
    ) -> OperateResult<Vec<model_platform_config::Model>> {
        info!(
            platformList.page.num = page_size.page.deref(),
            platformList.page.size = page_size.size.deref()
        );
        Ok(Entity::find()
            .with_pagination(page_size)
            .all(self.get_connect())
            .await?).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    let list = list.iter().map(|platform|(&platform.type_id)).collect::<SmallVec<[_;4]>>();
                    info!(platformList.len = list.len(),  platformList.platform.pType = ?list );
                });
            })
    }

    #[instrument(skip_all)]
    /// 获取全部平台type_id列表
    pub async fn find_all(&self) -> OperateResult<Vec<String>> {
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
        &self,
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
    pub async fn count_all(&self) -> OperateResult<u64> {
        let db = self.get_connect();
        model_platform_config::Entity::find()
            .count(db)
            .await
            .map_err(Into::into)
    }
}
