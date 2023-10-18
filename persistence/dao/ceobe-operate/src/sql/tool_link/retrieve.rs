use std::ops::Deref;

use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait},
    smallvec::SmallVec,
    sql_models::ceobe_operation::tool_link::models::model_tool_link::{
        self, Entity, FrontendToolLink,
    },
    tap::TapFallible,
};
use page_size::{database::WithPagination, request::Paginator};
use tracing::{info, instrument, Span};

use super::{OperateResult, ToolLinkOperate};

impl<'c, C> ToolLinkOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self))]
    /// 分页获取用户列表
    pub async fn find_list_paginator(
        &'c self, page_size: Paginator,
    ) -> OperateResult<Vec<model_tool_link::Model>> {
        info!(
            toolLink.page.num = page_size.page.deref(),
            toolLink.page.size = page_size.size.deref()
        );
        let db = self.get_connect();
        Ok(Entity::find()
            .with_pagination(page_size)
            .into_model::<model_tool_link::Model>()
            .all(db)
            .await?)
        .tap_ok(|list| {
            Span::current()
            .in_scope(||{
                let list = list.iter().map(|tool_link|(&tool_link.nickname)).collect::<SmallVec<[_;4]>>();
                info!(userList.len = list.len(),  userList.nicknames = ?list );
            })
            ;
        })
    }

    #[instrument(skip(self))]
    /// 获取用户列表
    pub async fn find_list(&'c self) -> OperateResult<Vec<FrontendToolLink>> {
        let db = self.get_connect();
        Ok(Entity::find()
            .into_model::<FrontendToolLink>()
            .all(db)
            .await?)
        .tap_ok(|list| {
            Span::current()
            .in_scope(||{
                let list = list.iter().map(|tool_link|(&tool_link.nickname)).collect::<SmallVec<[_;4]>>();
                info!(userList.len = list.len(),  userList.nicknames = ?list );
            })
            ;
        })
    }

    #[instrument(skip(self), ret)]
    /// 获取工具总数
    pub async fn get_user_total_number(&'c self) -> OperateResult<u64> {
        let db = self.get_connect();
        Entity::find().count(db).await.map_err(Into::into)
    }
}
