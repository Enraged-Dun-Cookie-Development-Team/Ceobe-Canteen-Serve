use std::ops::Deref;

use page_size::{database::OffsetLimit, request::PageSize};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect,
};
use smallvec::SmallVec;
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};
use crate::fetcher::datasource_config::{
    models::model_datasource_config,
    operate::retrieve::model_datasource_config::SingleDatasourceInfo,
};

impl FetcherDatasourceConfigSqlOperate {
    #[instrument(skip(db))]
    /// 分页获取全部数据源列表
    pub async fn find_datasource_list_by_page_size<'db, D>(
        db: &'db D, page_size: PageSize, platform: Option<String>,
        datasource: Option<String>,
    ) -> OperateResult<Vec<model_datasource_config::BackendDatasource>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            datasourceList.page.num = page_size.page.deref(),
            datasourceList.page.size = page_size.size.deref(),
            datasourceList.filter.platform = platform,
            datasourceList.filter.datasource = datasource,
        );
        let db = db.get_connect()?;
        let mut db_session = model_datasource_config::Entity::find();
        // 判断是否有传入值，来增加筛选项
        if let Some(platform_str) = platform {
            db_session = db_session.filter(
                model_datasource_config::Column::Platform.eq(platform_str),
            );
        }
        if let Some(datasource_str) = datasource {
            db_session = db_session.filter(
                model_datasource_config::Column::Datasource
                    .eq(datasource_str),
            );
        }
        let result = db_session
            .offset_limit(page_size)
            .into_model::<model_datasource_config::BackendDatasource>()
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
    pub async fn find_datasource_list_by_platform<'db, D>(
        db: &'db D, platform: String,
    ) -> OperateResult<Vec<model_datasource_config::DataSourceForFetcherConfig>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(datasourceList.platform = platform,);
        let db = db.get_connect()?;

        Ok(model_datasource_config::Entity::find()
            .filter(model_datasource_config::Column::Platform.eq(platform))
            .into_model::<model_datasource_config::DataSourceForFetcherConfig>()
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
    pub async fn find_datasource_type_list<'db, D>(
        db: &'db D,
    ) -> OperateResult<Vec<String>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        Ok(model_datasource_config::Entity::find()
            .select_only()
            .column(model_datasource_config::Column::Datasource)
            .group_by(model_datasource_config::Column::Datasource)
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
    pub async fn get_datasource_total_number<'db, D>(
        db: &'db D, platform: Option<String>, datasource: Option<String>,
    ) -> OperateResult<u64>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        let mut db_session = model_datasource_config::Entity::find();
        // 判断是否有传入值，来增加筛选项
        if let Some(platform_str) = platform {
            db_session = db_session.filter(
                model_datasource_config::Column::Platform.eq(platform_str),
            );
        }
        if let Some(datasource_str) = datasource {
            db_session = db_session.filter(
                model_datasource_config::Column::Datasource
                    .eq(datasource_str),
            );
        }
        db_session.count(db).await.map_err(Into::into)
    }
}
