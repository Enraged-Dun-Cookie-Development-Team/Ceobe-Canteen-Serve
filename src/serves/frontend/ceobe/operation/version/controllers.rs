use std::time::Duration;

use axum::extract::Query;
use axum_resp_result::{resp_result, resp_try, FlagWrap, MapReject};
use checker::CheckExtract;
use persistence::{
    ceobe_operate::{
        models::version::models::ReleaseVersion, ToCeobe, ToCeobeOperation,
    },
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use tracing::instrument;

use super::{
    error::FlagVersionRespResult,
    models::{
        AppVersion, DesktopVersion, OptionAppVersionCheckerPretreat,
        OptionDesktopVersionCheckerPretreat,
        OptionPluginVersionCheckerPretreat,
    },
    view::{AppVersionView, DesktopVersionView, PluginVersionView},
};
use crate::{
    router::CeobeOperationVersionFrontend,
    serves::frontend::ceobe::operation::version::{
        error::CeobeOperationVersionError, models::QueryReleaseVersion,
    },
};

impl CeobeOperationVersionFrontend {
    // 获取app对应版本信息
    #[instrument(skip(database, modify))]
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
                            .ceobe()
                            .operation()
                            .app_version()
                            .get_info_by_version(&version)
                            .await
                    }
                    None => {
                        database
                            .ceobe()
                            .operation()
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
                    db.ceobe()
                        .operation()
                        .plugin_version()
                        .get_info_by_version(version)
                        .await
                }
                None => {
                    db.ceobe()
                        .operation()
                        .plugin_version()
                        .get_newest_info()
                        .await
                }
            }?)?;
            Ok(FlagWrap::new(data.map(Into::into), extra))
        })
        .await
    }

    // 获取桌面端对应版本信息
    #[instrument(skip(database, modify))]
    pub async fn desktop_version(
        database: SqlDatabaseOperate,
        CheckExtract(DesktopVersion { version }): OptionDesktopVersionCheckerPretreat,
        mut modify: modify_cache::CheckModify,
    ) -> FlagVersionRespResult<DesktopVersionView> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        resp_try(async {
            let (data, extra) = modify.check_modify({
                match version {
                    Some(version) => {
                        database
                            .ceobe()
                            .operation()
                            .desktop_version()
                            .get_info_by_version(&version)
                            .await
                    }
                    None => {
                        database
                            .ceobe()
                            .operation()
                            .desktop_version()
                            .get_newest_info()
                            .await
                    }
                }?
            })?;
            Ok(FlagWrap::new(data.map(Into::into), extra))
        })
        .await
    }


}
