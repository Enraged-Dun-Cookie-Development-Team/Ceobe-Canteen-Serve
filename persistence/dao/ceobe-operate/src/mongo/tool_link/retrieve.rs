use std::ops::Deref;

use db_ops_prelude::{
    futures::StreamExt,
    mongo_connection::{MongoDbCollectionTrait, MongoDbError},
    mongodb::options::{CountOptions, FindOptions},
};
use page_size::request::Paginator;
use tracing::instrument;

use super::{OperateResult, ToolLinkOperate};
use crate::tool_link_mongodb::models::ToolLink;

impl<'db, Conn> ToolLinkOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ToolLink>,
{
    #[instrument(skip(self), name = "list")]
    pub async fn list(&'db self) -> OperateResult<Vec<ToolLink>> {
        let db = self.get_collection()?;

        let mut cursor =
            db.doing(|collection| collection.find(None, None)).await?;

        let mut result = Vec::<ToolLink>::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc.map_err(MongoDbError::from)?)
        }

        Ok(result)
    }

    #[instrument(skip(self), name = "page")]
    pub async fn page(
        &'db self, page_size: Paginator,
    ) -> OperateResult<Vec<ToolLink>> {
        let db = self.get_collection()?;

        let find_options = FindOptions::builder()
            .skip(Some(
                ((page_size.page.deref() - 1) * page_size.size.deref())
                    as u64,
            ))
            .limit(Some(*page_size.size.deref() as i64))
            .build();

        let mut cursor = db
            .doing(|collection| collection.find(None, find_options))
            .await?;

        let mut result = Vec::<ToolLink>::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc.map_err(MongoDbError::from)?)
        }

        Ok(result)
    }

    #[instrument(skip(self), name = "count")]
    pub async fn count(
        &'db self, page_size: Paginator,
    ) -> OperateResult<u64> {
        let db = self.get_collection()?;

        let count_options = CountOptions::builder()
            .skip(Some(
                ((page_size.page.deref() - 1) * page_size.size.deref())
                    as u64,
            ))
            .limit(Some(*page_size.size.deref() as u64))
            .build();

        let count = db
            .doing(|collection| {
                collection.count_documents(None, count_options)
            })
            .await
            .unwrap();

        Ok(count)
    }
}
