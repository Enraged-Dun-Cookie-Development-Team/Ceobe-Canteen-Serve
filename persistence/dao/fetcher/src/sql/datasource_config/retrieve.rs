use std::ops::Deref;

use tap::TapFallible;
use tracing::{info, instrument, Span};
use uuid::Uuid;

use db_ops_prelude::database_operates::NoConnect;
use db_ops_prelude::get_connect::GetDatabaseConnect;
use db_ops_prelude::get_zero_data_time;
use db_ops_prelude::sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect,
};
use db_ops_prelude::smallvec::SmallVec;
use db_ops_prelude::sql_models::fetcher::datasource_config::models::model_datasource_config::{BackendDatasource, Column, DatasourceBasicInfo, DataSourceForFetcherConfig, DatasourceId, DatasourceUuid, Entity, FrontendDatasource, Model, SingleDatasourceInfo};
use db_ops_prelude::sql_models::fetcher::datasource_config::models::model_datasource_config::DatasourcePlatform;
use page_size::{database::WithPagination, request::Paginator};

use super::{DatasourceOperate, OperateError, OperateResult};

impl DatasourceOperate<'_, NoConnect> {
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
            .filter(
                Condition::all()
                    .add(Column::Datasource.eq(datasource))
                    .add(Column::DbUniqueKey.eq(unique_key))
                    .add(Column::DeleteAt.ne(get_zero_data_time())),
            )
            .into_model()
            .one(db)
            .await?
            .ok_or(OperateError::DatasourcesNotFound)
    }

    /// 根据数据源与唯一值联合查询数据源信息
    pub async fn find_model_by_datasource_and_unique_key(
        db: &impl ConnectionTrait, datasource: &str, unique_key: &str,
    ) -> OperateResult<Model> {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Datasource.eq(datasource))
                    .add(Column::DbUniqueKey.eq(unique_key))
                    .add(Column::DeleteAt.eq(get_zero_data_time())),
            )
            .into_model()
            .one(db)
            .await?
            .ok_or(OperateError::DatasourcesNotFound)
    }
}

impl<'c, C> DatasourceOperate<'c, C>
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
            .filter(
                Condition::all()
                    .add(Column::Platform.eq(platform))
                    .add(Column::DeleteAt.eq(get_zero_data_time())),
            )
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
            .column(Column::JumpUrl)
            .column(Column::Platform)
            .column(Column::Datasource)
            .column(Column::DbUniqueKey)
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

    #[instrument(skip(self))]
    /// 根据数据源uuid获取数据源id
    pub async fn find_ids_by_uuids(
        &self, uuids: Vec<Uuid>,
    ) -> OperateResult<Vec<i32>> {
        let db = self.get_connect();
        Ok(Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(Column::UniqueId.is_in(uuids))
            .into_model::<DatasourceId>()
            .all(db)
            .await?
            .into_iter()
            .map(|datasource| datasource.id)
            .collect::<Vec<i32>>())
    }

    #[instrument(skip(self))]
    /// 根据数据源id获取数据源
    pub async fn find_basic_info_by_ids(
        &self, ids: &[i32],
    ) -> OperateResult<Vec<DatasourceBasicInfo>> {
        let db = self.get_connect();
        Ok(Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::Nickname)
            .column(Column::Avatar)
            .filter(Column::Id.is_in(ids.iter().copied()))
            .into_model::<DatasourceBasicInfo>()
            .all(db)
            .await?)
    }
}
