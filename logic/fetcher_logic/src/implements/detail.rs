use std::collections::{BTreeSet, HashMap};

use redis::{AsyncCommands, RedisError};
use redis_global::redis_key::fetcher::FetcherConfigKey;
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
    error::{LogicError, LogicResult},
    utils::GetOrCreate,
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
        } else {
            0
        };

        Ok(live_number)
    }

    /// 上传蹲饼器配置
    pub async fn upload_multi<'db, D>(
        db: &'db D, configs: impl IntoIterator<Item = BackEndFetcherConfig>,
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
        if upload_configs_uncheck.is_empty() {
            return Ok(());
        }
        use checker::LiteChecker;
        // 验证传入数据库数据的合法性
        let upload_config: Vec<FetcherConfig> =
            FetcherConfigVecChecker::lite_check(upload_configs_uncheck)
                .await?;

        // 判断平台是否存在
        let ctx = db.get_transaction().await?;

        if FetcherPlatformConfigSqlOperate::all_exist_by_type_ids(
            &ctx,
            upload_config.iter().map(|v| v.platform.as_str()),
        )
        .await?
            && FetcherDatasourceConfigSqlOperate::all_exist_by_id(
                &ctx,
                all_data_sources_set,
            )
            .await?
        {
            FetcherConfigSqlOperate::delete_by_all_platform(
                &ctx,
                upload_config.iter().map(|config| config.platform.as_str()),
            )
            .await?;

            FetcherConfigSqlOperate::create_multi(&ctx, upload_config).await?;
            // TODO:告诉调度器哪个平台更新了
        } else {
            Err(LogicError::PlatformNotFound)?;
        }
        ctx.submit().await?;
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

            let group = server_temp.get_or_try_create_with(
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
