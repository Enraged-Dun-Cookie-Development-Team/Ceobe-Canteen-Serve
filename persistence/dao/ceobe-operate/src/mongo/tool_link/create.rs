use db_ops_prelude::mongo_connection::MongoDbCollectionTrait;
use tracing::instrument;

use super::{OperateResult, ToolLinkOperate};
use crate::tool_link_mongodb::models::ToolLink;

impl<'db, Conn> ToolLinkOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ToolLink>,
{
    #[instrument(skip(self), name = "create")]
    pub async fn create(&'db self, tool_link: ToolLink) -> OperateResult<()> {
        let db = self.get_collection()?;

        db.doing(|collection| collection.insert_one(tool_link, None))
            .await?;

        Ok(())
    }
}
