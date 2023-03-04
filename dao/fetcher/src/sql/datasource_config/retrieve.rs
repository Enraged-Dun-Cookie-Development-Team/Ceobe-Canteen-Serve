use std::ops::Deref;

use db_ops_prelude::database_operates::NoConnect;
use db_ops_prelude::get_connect::GetDatabaseConnect;
use db_ops_prelude::get_zero_data_time;
use page_size::{database::WithPagination, request::Paginator};
use db_ops_prelude::sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, StreamTrait,
};
use db_ops_prelude::sql_models::fetcher::datasource_config::models::model_datasource_config::{Entity, Column, Model, BackendDatasource, DataSourceForFetcherConfig, DatasourceUuid, SingleDatasourceInfo, FrontendDatasource};
use db_ops_prelude::smallvec::SmallVec;
use db_ops_prelude::sql_models::fetcher::datasource_config::models::model_datasource_config::DatasourcePlatform;
use tap::TapFallible;
use tracing::{info, instrument, Span};
use uuid::Uuid;
use super::{
    OperateError, OperateResult, DatasourceOperate,
};

impl DatasourceOperate<'_, NoConnect> {
    pub async fn find_platform_by_id<'s, 'db, C>(
        db: &'db C, id: i32,
    ) -> OperateResult<DatasourcePlatform>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
        Entity::find_by_id(id)
            .select_only()
            .column(Column::Platform)
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .into_model()
            .one(db)
            .await?
            .ok_or(OperateError::DatasourceNotFound(id))
    }

    pub async fn find_delete_model_by_datasource_and_unique_key<'s, 'db, C>(
        db: &'db C, datasource: &str, unique_key: &str,
    ) -> OperateResult<Model>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
        Entity::find()
            .filter(Column::Datasource.eq(datasource))
            .filter(Column::DbUniqueKey.eq(unique_key))
            .filter(Column::DeleteAt.ne(get_zero_data_time()))
            .into_model()
            .one(db)
            .await?
            .ok_or(OperateError::DatasourcesNotFound)
    }
}

impl<'c, C> DatasourceOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
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
                    let list = list.iter().map(|datasource|(&datasource.nickname)).collect::<SmallVec<[_;4]>>();
                    info!(datasourceList.len = list.len(),  datasourceList.datasource = ?list );
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
    /// 获取全部数据源UUID列表
    pub async fn find_all_uuid(&self) -> OperateResult<Vec<Uuid>> {
        let db = self.get_connect();

        Ok(Entity::find()
            .select_only()
            .column(Column::UniqueId)
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .into_model::<DatasourceUuid>()
            .all(db)
            .await?
            .into_iter()
            .map(|item| item.unique_id)
            .collect::<Vec<Uuid>>()
            ).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    info!(datasourceUuid.len = list.len(),  datasourceUuidList.unique_ids = ?list );
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

    #[instrument(skip(self))]
    /// 获取全部携带给前台唯一标识的数据源列表
    pub async fn find_all_with_unique_id(
        &self,
    ) -> OperateResult<Vec<FrontendDatasource>> {
        let db = self.get_connect();
        let result = Entity::find()
            .select_only()
            .column(Column::Nickname)
            .column(Column::Avatar)
            .column(Column::UniqueId)
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .into_model::<FrontendDatasource>()
            .all(db)
            .await?;

        Ok(result).tap_ok(|list| {
                Span::current()
                .in_scope(||{
                    let list = list.iter().map(|datasource|(&datasource.nickname)).collect::<SmallVec<[_;4]>>();
                    info!(datasourceList.len = list.len(),  datasourceList.datasource = ?list );
                });
            })
    }
}
