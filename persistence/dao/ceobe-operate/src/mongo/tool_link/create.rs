use tracing::instrument;

use db_ops_prelude::futures::StreamExt;
use db_ops_prelude::mongo_connection::{MongoDbCollectionTrait, MongoDbError};

use crate::tool_link_mongodb::models::ToolLink;

use super::{OperateResult, ToolLinkOperate};

impl<'db, Conn> ToolLinkOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ToolLink>,
{
    #[instrument(skip(self), name = "create")]
    pub async fn create(
        &'db self, tool_link: ToolLink,
    ) -> OperateResult<()> {
        let db = self.get_collection()?;

        db.doing(|collect| collect.insert_one(tool_link, None))
            .await?;

        Ok(())
    }

    #[instrument(skip(self), name = "list")]
    pub async fn list(
        &'db self,
    ) -> OperateResult<Vec<ToolLink>> {
        let db = self.get_collection()?;

        let mut cursor = db.doing(|collect| collect.find(None, None))
            .await.unwrap();

        let mut result = Vec::<ToolLink>::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc.map_err(MongoDbError::from)?)
        }

        Ok(result)
    }
}