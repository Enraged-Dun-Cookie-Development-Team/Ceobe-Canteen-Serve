use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::{self, ceobe::operation::plugin_version::{PluginVersion, PluginVersionChecked}},
    RecordUnit,
};
use db_ops_prelude::mongo_models::SetRecordUnit;
use mongo_models::RecordUnitUpdater;
use tracing::{info, instrument};
use super::{Checked, OperateResult, PluginVersionOperate};
impl<'db, Conn> PluginVersionOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, PluginVersion>,
{
    #[instrument(skip(self), name = "insert-new")]
    pub async fn update_new(
        &'db self, version: Checked,
    ) -> OperateResult<()> {
        info!(newPlugin.version = %version.version);
        let db = self.get_collection()?;

        let plugin_version =
        version.into_with_time_record(RecordUnit::new());

        // version can not be the same even is delete
        PluginVersionOperate::verify_version(plugin_version.version, &db)
            .await?;
        // update

        db.doing(|collect| collect.insert_one(plugin_version, None))
            .await?;

        Ok(())
    }
}
