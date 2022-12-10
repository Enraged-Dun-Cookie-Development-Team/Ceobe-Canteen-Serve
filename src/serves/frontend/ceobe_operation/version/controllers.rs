use std::time::Duration;

use checker::CheckExtract;
use mongo_migration::mongo_connection::MongoConnect;
use orm_migrate::sql_connection::SqlConnect;
use resp_result::{resp_try, FlagWrap};
use tracing::instrument;

use super::{
    error::FlagVersionRespResult,
    models::{
        AppVersion, OptionAppVersionCheckerPretreat,
        OptionPluginVersionCheckerPretreat,
    },
    view::{AppVersionView, PluginVersionView},
};
use crate::{
    models::{
        mongo::plugin_version::operates::PluginDbOperation,
        sql::app_version::operate::CeobeOperationAppVersionSqlOperate,
    },
    router::CeobeOperationVersionFrontend,
};

impl CeobeOperationVersionFrontend {
    // 获取app对应版本信息
    #[instrument(skip(db, modify))]
    pub async fn app_version(
        db: SqlConnect,
        CheckExtract(AppVersion { version }): OptionAppVersionCheckerPretreat,
        mut modify: modify_cache::CheckModify,
    ) -> FlagVersionRespResult<AppVersionView> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        resp_try(async {

            let (data,extra) = modify.check_modify(
                match version {
            Some(version) => {
                    CeobeOperationAppVersionSqlOperate::get_app_version_info_by_version(&db,&version).await
                }
                None => {
                    CeobeOperationAppVersionSqlOperate::get_newest_app_version_info(&db).await
                }
            }?
        )?;
        Ok(FlagWrap::new(data.map(Into::into),extra))
    }).await
    }

    // 获取插件端对应版本信息
    #[instrument(skip(db, modify))]
    pub async fn plugin_version(
        db: MongoConnect,
        CheckExtract(version): OptionPluginVersionCheckerPretreat,
        mut modify: modify_cache::CheckModify,
    ) -> FlagVersionRespResult<PluginVersionView> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        let version = version.version;

        resp_try(async {
            let (data, extra) = modify.check_modify(match version {
                Some(version) => {
                    PluginDbOperation::get_plugin_version_info_by_version(
                        &db, version,
                    )
                    .await
                }
                None => {
                    PluginDbOperation::get_newest_plugin_version_info(&db)
                        .await
                }
            }?)?;
            Ok(FlagWrap::new(data.map(Into::into), extra))
        })
        .await
    }
}
