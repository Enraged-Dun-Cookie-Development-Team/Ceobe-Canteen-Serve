use std::ops::Deref;
use crate::fetcher::platform_config::operate::retrieve::model_platform_config::SinglePlatformInfo;
use page_size::{request::PageSize, database::OffsetLimit};
use sea_orm::{DbErr, ConnectionTrait, EntityTrait, QuerySelect, PaginatorTrait};
use smallvec::SmallVec;
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::TapFallible;
use super::OperateResult;
use tracing::{instrument, info, Span};

use crate::fetcher::platform_config::models::model_platform_config;

use super::FetcherPlatformConfigSqlOperate;

impl FetcherPlatformConfigSqlOperate {
    #[instrument(skip(db))]
    /// 分页获取全部平台列表
    pub async fn find_platform_list_by_page_size<'db, D>(
        db: &'db D, page_size: PageSize,
    ) -> OperateResult<Vec<model_platform_config::Model>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            platformList.page.num = page_size.page.deref(),
            platformList.page.size = page_size.size.deref()
        );
        let db = db.get_connect()?;
        Ok(model_platform_config::Entity::find()
            .offset_limit(page_size)
            .into_model::<model_platform_config::Model>()
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
    pub async fn find_platform_list<'db, D>(
        db: &'db D
    ) -> OperateResult<Vec<String>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        Ok(model_platform_config::Entity::find()
            .select_only()
            .column(model_platform_config::Column::TypeId)
            .into_model::<SinglePlatformInfo>()
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

    #[instrument(skip(db), ret)]
    /// 获取平台总数
    pub async fn get_platform_total_number<'db, D>(
        db: &'db D,
    ) -> OperateResult<u64>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        model_platform_config::Entity::find().count(db).await.map_err(Into::into)
    }
}
