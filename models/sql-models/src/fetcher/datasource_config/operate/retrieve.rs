use std::ops::Deref;

use page_size::{database::WithPagination, request::Paginator};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect,
};
use smallvec::SmallVec;
use sql_connection::database_traits::{
    database_operates::NoConnect, get_connect::GetDatabaseConnect,
};
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{
    super::models::model_datasource_config::DatasourcePlatform, Datasource,
    OperateError, OperateResult,
};
use crate::{
    fetcher::datasource_config::{
        models::model_datasource_config::{
            self, BackendDatasource, Column, DataSourceForFetcherConfig,
            Entity, Model,
        },
        operate::retrieve::model_datasource_config::SingleDatasourceInfo,
    },
    get_zero_data_time,
};

impl Datasource<'_, NoConnect> {
    pub async fn find_platform_by_id(
        db: &impl ConnectionTrait, id: i32,
    ) -> OperateResult<DatasourcePlatform> {
        Entity::find_by_id(id)
            .select_only()
            .column(Column::Platform)
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .into_model()
            .one(db)
            .await?
            .ok_or(OperateError::DatasourceNotFound(id))
    }

    pub async fn find_delete_model_by_datasource_and_unique_key(
        db: &impl ConnectionTrait, datasource: &str, unique_key: &str,
    ) -> OperateResult<Model> {
        Entity::find()
            .filter(Column::Datasource.eq(datasource))
            .filter(Column::DbUniqueKey.eq(unique_key))
            .filter(Column::DeleteAt.ne(get_zero_data_time()))
            .into_model()
            .one(db)
            .await?
            .ok_or(OperateError::DatasourceNotExist)
    }
}

impl<'c, C> Datasource<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self))]
    /// 分页获取全部数据源列表
    pub async fn find_all_with_paginator(
        &self, page_size: Paginator, platform: Option<String>,
        datasource: Option<String>,
    ) -> OperateResult<Vec<BackendDatasource>> {
        info!(
            datasourceList.page.num = page_size.page.deref(),
            datasourceList.page.size = page_size.size.deref(),
            datasourceList.filter.platform = platform,
            datasourceList.filter.datasource = datasource,
        );
        let db = self.get_connect();
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
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
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

    #[instrument(skip(self))]
    /// 获取单个平台下的全部数据源列表
    pub async fn find_by_platform(
        &self, platform: &str,
    ) -> OperateResult<Vec<DataSourceForFetcherConfig>> {
        info!(datasourceList.platform = platform,);
        let db = self.get_connect();

        Ok(Entity::find()
            .filter(Column::Platform.eq(platform))
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
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

    #[instrument(skip(self))]
    /// 获取全部数据源类型列表（如：B站动态、B站视频、网易云专辑、
    /// 网易云歌手等）
    pub async fn find_all_type(&self) -> OperateResult<Vec<String>> {
        let db = self.get_connect();
        Ok(Entity::find()
            .select_only()
            .column(Column::Datasource)
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
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

    #[instrument(skip(self), ret)]
    /// 获取数据源总数
    pub async fn count(
        &self, platform: Option<String>, datasource: Option<String>,
    ) -> OperateResult<u64> {
        let db = self.get_connect();
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
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .count(db)
            .await
            .map_err(Into::into)
    }
}
