use std::time::Duration;

use abstract_database::ceobe::ToCeobe;
use ceobe_operate::ToCeobeOperation;
use checker::CheckExtract;
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;
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
use crate::router::CeobeOperationVersionFrontend;

impl CeobeOperationVersionFrontend {
    // 获取app对应版本信息
    // #[instrument(skip(db, modify))]
    pub async fn app_version(
        database: SqlDatabaseOperate,
        CheckExtract(AppVersion { version }): OptionAppVersionCheckerPretreat,
        mut modify: modify_cache::CheckModify,
    ) -> FlagVersionRespResult<AppVersionView> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        resp_try(async {
            let (data, extra) = modify.check_modify({
                match version {
                    Some(version) => {
                        database
                            .ceobe().operation()
                            .app_version()
                            .get_info_by_version(&version)
                            .await
                    }
                    None => {
                        database
                            .ceobe().operation()
                            .app_version()
                            .get_newest_info()
                            .await
                    }
                }?
            })?;
            Ok(FlagWrap::new(data.map(Into::into), extra))
        })
        .await
    }

    // 获取插件端对应版本信息
    #[instrument(skip(db, modify))]
    pub async fn plugin_version(
        db: MongoDatabaseOperate,
        CheckExtract(version): OptionPluginVersionCheckerPretreat,
        mut modify: modify_cache::CheckModify,
    ) -> FlagVersionRespResult<PluginVersionView> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        let version = version.version;

        resp_try(async {
            let (data, extra) = modify.check_modify(match version {
                Some(version) => {
                    db.ceobe().operation()
                        .plugin_version()
                        .get_info_by_version(version)
                        .await
                }
                None => {
                    db.ceobe().operation()
                        .plugin_version()
                        .get_newest_info()
                        .await
                }
            }?)?;
            Ok(FlagWrap::new(data.map(Into::into), extra))
        })
        .await
    }
}
