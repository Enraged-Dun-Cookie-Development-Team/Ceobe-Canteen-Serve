use std::borrow::Cow;

use axum_resp_result::{resp_result, RespError, RespResult};
use bson::doc;
use futures::TryStreamExt;
use persistence::{
    ceobe_cookie::ToCeobe,
    ceobe_operate::{
        app_version, desktop_version,
        plugin_version::{PluginVersion, PluginVersionChecked},
        ToCeobeOperation,
    },
    help_crates::sea_orm::{EntityTrait, QueryOrder},
    mongodb::{
        database_traits::OperateTrait, mongodb,
        mongodb::options::FindOptions, MongoConnect, MongoDatabaseOperate,
        MongoDbError,
    },
    mysql::{sea_orm::DbErr, SqlConnect, SqlDatabaseOperate},
    operate::{DatabaseOperate, GetDatabaseConnect},
};
use qq_channel_warning::{
    LogRequest, LogType, QqChannelGrpcService, QqChannelGrpcState,
};
use tracing::{error, info, instrument, warn};
pub async fn migrate_version(qq_channel: QqChannelGrpcState) {
    let qq_channel = QqChannelGrpcService::new_with_uri(qq_channel).await;
    let mut qq_channel = match qq_channel {
        Ok(qq_channel_warning) => qq_channel_warning,
        Err(err) => {
            error!(Action="Migrate Version", Error = %err);
            return;
        }
    };

    let a = raw_migrate_version(
        DatabaseOperate::new(SqlConnect),
        DatabaseOperate::new(MongoConnect),
    )
    .await;
    match a {
        RespResult::Success(_) => {}
        RespResult::Err(err) => {
            qq_channel
                .send_logger(
                    LogRequest::builder()
                        .info(format!("版本迁移时异常: {err}"))
                        .manual()
                        .level(LogType::Error)
                        .build(),
                )
                .await
                .ok();
        }
    }
}

#[resp_result]
#[instrument(skip_all)]
pub async fn raw_migrate_version(
    sql: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
) -> Result<(), Error> {
    let mongo_conn = mongo.get_connect();

    let sql = sql.get_connect();
    // 迁移 AppVersion
    let all_app_version = app_version::Entity::find()
        .order_by_asc(app_version::Column::CreateAt)
        .all(sql)
        .await?;

    info!(
        Action = "Migrate Version",
        Platform = "移动端",
        Number = all_app_version.len()
    );
    for ver in all_app_version {
        let crr_ver = ver.version.clone();
        let result = mongo
            .ceobe()
            .operation()
            .release_version()
            .create()
            .one(app_version::Checked::from(ver))
            .await;

        if let Err(err) = result {
            warn!(
                Action = "迁移旧版Version",
                Version = crr_ver,
                Platform = "移动端",
                Error = %err
            );
        }
    }

    let all_desktop_version = desktop_version::Entity::find()
        .order_by_asc(desktop_version::Column::CreateAt)
        .all(sql)
        .await?;
    info!(
        Action = "Migrate Version",
        Platform = "桌面端",
        Number = all_desktop_version.len()
    );
    for ver in all_desktop_version {
        let crr_ver = ver.version.clone();
        let result = mongo
            .ceobe()
            .operation()
            .release_version()
            .create()
            .one(desktop_version::Checked::from(ver))
            .await;

        if let Err(err) = result {
            warn!(
                Action = "迁移旧版Version",
                Version = crr_ver,
                Platform = "桌面端",
                Error = %err
            );
        }
    }

    let plugin_version_collection =
        mongo_conn.get_collection::<PluginVersion>()?;

    let all_plugin_version = plugin_version_collection
        .find(
            doc! {},
            FindOptions::builder()
                .sort(doc! {"time_record.create_at":1i32})
                .build(),
        )
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    info!(
        Action = "Migrate Version",
        Platform = "插件端",
        Number = all_plugin_version.len()
    );
    for ver in all_plugin_version {
        let crr_ver = ver.version;
        let result = mongo
            .ceobe()
            .operation()
            .release_version()
            .create()
            .one(PluginVersionChecked::from(ver))
            .await;

        if let Err(err) = result {
            warn!(
                Action = "迁移旧版Version",
                Version = %crr_ver,
                Platform = "插件端",
                Error = %err
            );
        }
    }
    info!(Action = "Migrate Version", Status = "完成");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Db(#[from] DbErr),
    #[error(transparent)]
    Mongo(#[from] MongoDbError),
    #[error(transparent)]
    MongoErr(#[from] mongodb::error::Error),
}

impl RespError for Error {
    type ExtraMessage = i32;

    fn log_message(&self) -> Cow<'_, str> { self.to_string().into() }

    fn extra_message(&self) -> Self::ExtraMessage { 0 }
}
