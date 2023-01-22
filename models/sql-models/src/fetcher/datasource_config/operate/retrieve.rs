use std::ops::Deref;

use page_size::{database::WithPagination, request::Paginator};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait,
    PaginatorTrait, QueryFilter, QuerySelect,
};
use smallvec::SmallVec;
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{
    super::models::model_datasource_config::DatasourcePlatform,
    FetcherDatasourceConfigSqlOperate, OperateError, OperateResult,
};
use crate::fetcher::datasource_config::{
    models::model_datasource_config::{
        self, BackendDatasource, Column, DataSourceForFetcherConfig, Entity,
    },
    operate::retrieve::model_datasource_config::SingleDatasourceInfo,
};

impl FetcherDatasourceConfigSqlOperate {
    pub async fn find_platform_by_id(
        &self, db: &impl ConnectionTrait, id: i32,
    ) -> OperateResult<DatasourcePlatform> {
        Entity::find_by_id(id)
            .select_only()
            .column(Column::Platform)
            .into_model()
            .one(db)
            .await?
            .ok_or(OperateError::DatasourceNotFound(id))
    }

    #[instrument(skip(db))]
    /// 分页获取全部数据源列表
    pub async fn find_all_with_paginator<'db, D>(
        db: &'db D, page_size: Paginator, platform: Option<String>,
        datasource: Option<String>,
    ) -> OperateResult<Vec<BackendDatasource>>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            datasourceList.page.num = page_size.page.deref(),
            datasourceList.page.size = page_size.size.deref(),
            datasourceList.filter.platform = platform,
            datasourceList.filter.datasource = datasource,
        );
        let db = db.get_connect();
        let result = Entity::find()
            .filter(
                Condition::all()
                    .add_option(platform.map(|platform_str| {
                        Column::Platform.eq(platform_str)
                    }))
                    .add_option(datasource.map(|datasource_str| {
                        Column::Datasource.eq(datasource_str)
                    })),
            )
            .with_pagination(page_size)
            .into_model::<BackendDatasource>()
            .all(db)
            .await?;

        Ok(result).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    let list = list.iter().map(|platform|(&platform.nickname)).collect::<SmallVec<[_;4]>>();
                    info!(platformList.len = list.len(),  platformList.platform.pType = ?list );
                });
            })
    }

    #[instrument(skip(db))]
    /// 获取单个平台下的全部数据源列表
    pub async fn find_by_platform<'db, D>(
        db: &'db D, platform: &str,
    ) -> OperateResult<Vec<DataSourceForFetcherConfig>>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(datasourceList.platform = platform,);
        let db = db.get_connect();

        Ok(Entity::find()
            .filter(Column::Platform.eq(platform))
            .into_model::<DataSourceForFetcherConfig>()
            .all(db)
            .await?).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    let list = list.iter().map(|platform|(&platform.nickname)).collect::<SmallVec<[_;4]>>();
                    info!(platformList.len = list.len(),  platformList.platform.pType = ?list );
                });
            })
    }

    #[instrument(skip(db))]
    /// 获取全部数据源类型列表（如：B站动态、B站视频、网易云专辑、
    /// 网易云歌手等）
    pub async fn find_all_type<'db, D>(
        db: &'db D,
    ) -> OperateResult<Vec<String>>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect();
        Ok(Entity::find()
            .select_only()
            .column(Column::Datasource)
            .group_by(Column::Datasource)
            .into_model::<SingleDatasourceInfo>()
            .all(db)
            .await?
            .into_iter()
            .map(|item| item.datasource)
            .collect::<Vec<String>>()
            ).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    info!(datasourceTypeList.len = list.len(),  datasourceTypeList.dType = ?list );
                });
            })
    }

    #[instrument(skip(db), ret)]
    /// 获取数据源总数
    pub async fn count<'db, D>(
        db: &'db D, platform: Option<String>, datasource: Option<String>,
    ) -> OperateResult<u64>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect();
        Entity::find()
            .filter(
                Condition::all()
                    .add_option(platform.map(|platform_str| {
                        Column::Platform.eq(platform_str)
                    }))
                    .add_option(datasource.map(|datasource_str| {
                        Column::Datasource.eq(datasource_str)
                    })),
            )
            .count(db)
            .await
            .map_err(Into::into)
    }
}
