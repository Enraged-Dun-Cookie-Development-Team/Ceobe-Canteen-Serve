use std::ops::Deref;

use page_size::{request::PageSize, database::OffsetLimit};
use sea_orm::{DbErr, ConnectionTrait, EntityTrait};
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
    /// 获取全部平台列表
    pub async fn find_platform_list<'db, D>(
        db: &'db D
    ) -> OperateResult<Vec<model_platform_config::Model>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        Ok(model_platform_config::Entity::find()
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
}
