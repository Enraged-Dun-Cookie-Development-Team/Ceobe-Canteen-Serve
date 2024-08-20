use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongodb::bson::{self, doc, to_bson},
};
use tracing::instrument;

use super::{OperateResult, ToolLinkOperate};
use crate::tool_link_mongodb::models::{ToolLink, ToolLinkUpdate};

impl<'db, Conn> ToolLinkOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ToolLink>,
{
    #[instrument(skip(self), name = "list")]
    pub async fn update(&'db self, tool_link: ToolLink) -> OperateResult<()> {
        let db = self.get_collection()?;
        let query = doc! {"id": tool_link.id};
        let update = to_bson(&ToolLinkUpdate::from(tool_link))?;
        db.doing(|collection| {
            collection.update_one(
                query,
                doc! {
                    "$set": update
                },
                None,
            )
        })
        .await?;

        Ok(())
    }
}
