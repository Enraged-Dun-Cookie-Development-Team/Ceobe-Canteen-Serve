use std::collections::{BTreeSet, HashMap};

use checker::prefabs::post_checker::PostChecker;
use redis::{AsyncCommands, RedisError};
use redis_global::redis_key::fetcher::FetcherConfigKey;
use scheduler_notifier::SchedulerNotifier;
use sql_models::{
    fetcher::{
        config::{
            checkers::config_data::{
                FetcherConfig, FetcherConfigUncheck, FetcherConfigVecChecker,
            },
            models::model_config::Model as FetcherConfigModel,
            operate::FetcherConfigSqlOperate,
        },
        datasource_config::operate::FetcherDatasourceConfigSqlOperate,
        platform_config::operate::FetcherPlatformConfigSqlOperate,
    },
    sql_connection::{
        database_traits::get_connect::{
            GetDatabaseConnect, GetDatabaseTransaction,
            GetMutDatabaseConnect, TransactionOps,
        },
        sea_orm::{ConnectionTrait, DbErr},
    },
};

use super::FetcherConfigLogic;
use crate::{
    checkers::check_platform_same::PlatformSameChecker,
    error::{LogicError, LogicResult},
    utils::{GetOrCreate, TrueOrError},
    view::{BackEndFetcherConfig, Group, Server},
};

impl FetcherConfigLogic {
    /// 获取蹲饼器最大存活数量
    pub async fn get_max_live_number<'client, C>(
        client: &'client mut C,
    ) -> LogicResult<i8>
    where
        C: GetMutDatabaseConnect<Error = RedisError> + 'client,
        C::Connect<'client>: AsyncCommands,
    {
        let con = client.mut_connect()?;

        // 判断redis key存在，如果不存在则默认没有蹲饼器
        let live_number = if con.exists(FetcherConfigKey::LIVE_NUMBER).await?
        {
            // 获取key的值
            con.get(FetcherConfigKey::LIVE_NUMBER).await?
        }
        else {
            0
        };

        Ok(live_number)
    }

    /// 上传蹲饼器配置
    pub async fn upload_multi<'db, D>(
        notifier: &SchedulerNotifier, db: &'db D,
        configs: impl IntoIterator<Item = BackEndFetcherConfig>,
    ) -> LogicResult<()>
    where
        D: GetDatabaseTransaction<Error = DbErr> + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        let mut upload_configs_uncheck = Vec::<FetcherConfigUncheck>::new();

        let mut all_data_sources_set = BTreeSet::new();

        // 将上传数据格式换成unchecked结构体
        for BackEndFetcherConfig { number, server } in configs {
            for (count, Server { groups }) in server.into_iter().enumerate() {
                for Group {
                    name,
                    platform,
                    data_source,
                    interval,
                    interval_by_time_range,
                } in groups
                {
                    for id in data_source {
                        all_data_sources_set.insert(id);

                        upload_configs_uncheck.push(
                            FetcherConfigUncheck::builder()
                                .live_number(number)
                                .fetcher_count(count)
                                .group_name(name.clone())
                                .platform(platform.clone())
                                .datasource_id(id)
                                .interval(interval)
                                .interval_by_time_range(serde_json::to_value(
                                    &interval_by_time_range,
                                )?)
                                .build(),
                        )
                    }
                }
            }
        }

        type FetcherConfigSliceChecker = PostChecker<
            FetcherConfigVecChecker,
            // 追加检查，确认所有的platform 全部一致
            PlatformSameChecker,
            LogicError,
        >;

        use checker::LiteChecker;
        // 验证传入数据库数据的合法性
        let upload_config: Vec<FetcherConfig> =
            FetcherConfigSliceChecker::lite_check(upload_configs_uncheck)
                .await?;
        // 取出第一个，如果没有，那将无效果，返回
        let Some(platform) = upload_config.first().map(FetcherConfig::get_platform_type_id).map(ToOwned::to_owned) else{
            return Ok(());
        };

        let ctx = db.get_transaction().await?;

        let platform_exist =
            FetcherPlatformConfigSqlOperate::exist_by_type_id(
                &ctx, &platform,
            )
            .await?;
        let all_datasource_exist =
            FetcherDatasourceConfigSqlOperate::all_exist_by_id(
                &ctx,
                all_data_sources_set,
            )
            .await?;

        // 指定平台与数据源均存在
        (platform_exist && all_datasource_exist)
            .true_or_with(|| LogicError::PlatformNotFound)?;
        // 清除指定 platform 下全部 config
        FetcherConfigSqlOperate::delete_by_platform(&ctx, &platform).await?;
        // 创建config
        FetcherConfigSqlOperate::create_multi(&ctx, upload_config).await?;
        ctx.submit().await?;
        notifier.notify_platform_update(platform).await;
        Ok(())
    }

    /// 获取蹲饼器配置
    pub async fn get_by_platform<'db, D>(
        db: &'db D, platform: &str,
    ) -> LogicResult<Vec<BackEndFetcherConfig>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let configs_in_db =
            FetcherConfigSqlOperate::find_all_by_platform(db, platform)
                .await?;

        let mut configs =
            HashMap::<i8, HashMap<i8, HashMap<String, Group>>>::new();
        for config @ FetcherConfigModel {
            live_number,
            fetcher_count,
            datasource_id,
            ..
        } in configs_in_db
        {
            let server_temp = configs
                .get_mut_or_default(live_number)
                .get_mut_or_default(fetcher_count - 1);

            let group = server_temp.get_mut_or_try_create_with(
                config.group_name.to_owned(),
                || Group::try_from(config),
            )?;
            group.data_source.push(datasource_id);
        }

        let configs = configs
            .into_iter()
            .map(|(number, mut server)| {
                let servers = (0..number).map(|idx| {
                    server
                        .remove(&idx)
                        .into_iter()
                        .flat_map(|map| {
                            map.into_iter().map(|(_, group)| group)
                        })
                        .into()
                });
                BackEndFetcherConfig::new(number, servers)
            })
            .collect();
        Ok(configs)
    }
}
