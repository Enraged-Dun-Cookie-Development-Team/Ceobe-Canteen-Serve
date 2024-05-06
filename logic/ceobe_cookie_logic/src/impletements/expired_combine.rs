use std::{sync::Arc, time::Duration};

use persistence::{
    fetcher::{
        datasource_combination::{
            DatasourceCombinationOperate, ToDatasourceCombination,
        },
        ToFetcher,
    },
    help_crates::tracing::error,
    mysql::SqlConnect,
    operate::{DatabaseOperate, GetDatabaseConnect, GetMutDatabaseConnect},
    redis::RedisConnect,
};
use qiniu_service::model::DeleteObjectName;
use qq_channel_warning::{
    LogRequest, LogType, QqChannelGrpcService, QqChannelGrpcState,
};
use redis_global::{redis_key::cookie_list::NewCombIdInfo, RedisTypeBind};
use tokio::time::interval;

use crate::{error::LogicError, impletements::CeobeCookieLogic};

impl CeobeCookieLogic {
    pub async fn remove_expired_combine_id(
        sql: SqlConnect, redis: RedisConnect,
        qiniu: Arc<ceobe_qiniu_upload::Manager>,
        qq_channel: QqChannelGrpcState,
    ) {
        let Ok(mut qq_channel) = QqChannelGrpcService::new(qq_channel).await
        else {
            error!(
                r#type = "ScheduleTaskError",
                reason = "QQChannel Connection cannot establish"
            );
            return;
        };
        let mut interval = interval(Duration::from_secs(60 * 60 * 24 * 7));

        loop {
            interval.tick().await;

            let mut logic_error = Option::<LogicError>::None;
            for _ in 0..3 {
                let Err(err) = Self::inner_remove_expired_combine_id(
                    DatabaseOperate::new(sql.clone()),
                    DatabaseOperate::new(redis.clone()),
                    qiniu.clone(),
                )
                .await
                else {
                    break;
                };

                let _ = logic_error.insert(err);
            }

            if let Some(error) = logic_error {
                error!(
                    r#type = "ScheduleTaskError",
                    reason = "remove expired CombineId Failure",
                    error = %error
                );

                let Err(err) = qq_channel
                    .send_logger(
                        LogRequest::builder()
                            .level(LogType::Error)
                            .info("定时清理过期组合饼ID异常".into())
                            .extra(error.to_string())
                            .build(),
                    )
                    .await
                else {
                    continue;
                };
                error!(
                    r#type = "ScheduleTaskError",
                    reason = "Send QQ channel Failure",
                     error = %err
                );
            }
        }
    }

    async fn inner_remove_expired_combine_id(
        sql: DatabaseOperate<SqlConnect>,
        mut redis: DatabaseOperate<RedisConnect>,
        qiniu: Arc<ceobe_qiniu_upload::Manager>,
    ) -> Result<(), crate::error::LogicError> {
        // get all expired id
        let expired_ids = sql
            .fetcher()
            .datasource_combination()
            .find_expired(60)
            .await?;

        // clean ids in the redis
        NewCombIdInfo
            .bind(redis.mut_connect())
            .remove_all(expired_ids.as_slice())
            .await?;

        // delete ids in the qiniu collect
        qiniu
            .delete_many(
                expired_ids
                    .iter()
                    .cloned()
                    .map(DeleteObjectName::new)
                    .collect(),
            )
            // ignore qiniu error
            .await
            .ok();

        // clear expired id in the sql
        DatasourceCombinationOperate::delete_by_datasource(
            sql.get_connect(),
            expired_ids,
        )
        .await?;

        Ok(())
    }
}
