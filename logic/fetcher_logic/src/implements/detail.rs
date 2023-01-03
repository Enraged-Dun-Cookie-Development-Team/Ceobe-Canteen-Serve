use std::collections::{BTreeSet, HashMap};

use checker::ToCheckRequire;
use redis::{RedisError, AsyncCommands};
use redis_global::redis_key::fetcher::FetcherConfigKey;
use sql_models::{sql_connection::{database_traits::get_connect::{GetMutDatabaseConnect, GetDatabaseTransaction, GetDatabaseConnect}, sea_orm::{DbErr, ConnectionTrait}}, fetcher::{config::{checkers::config_data::{FetcherConfig, FetcherConfigVecChecker}, operate::FetcherConfigSqlOperate}, platform_config::operate::FetcherPlatformConfigSqlOperate, datasource_config::operate::FetcherDatasourceConfigSqlOperate}};
use sql_models::fetcher::config::{checkers::config_data::FetcherConfigUncheck , models::model_config::Model as FetcherConfigModel,};
use crate::{error::{LogicResult, LogicError}, view::{BackFetcherConfig, Server, Group}, utils::GetOrCreate};
use checker::Checker;
use super::FetcherConfigLogic;

impl FetcherConfigLogic {
    /// 获取蹲饼器最大存活数量
    pub async fn get_cookie_fetcher_max_live_number<'client, C>(
        client: &'client mut C,
    ) -> LogicResult<i8>
    where
        C: GetMutDatabaseConnect<Error = RedisError> + 'client,
        C::Connect<'client>: AsyncCommands,
    {
        let con = client.mut_connect()?;
        let mut live_number = 0;
        // 判断redis key存在，如果不存在则默认没有蹲饼器
        if con.exists(FetcherConfigKey::LIVE_NUMBER).await? {
            // 获取key的值
            live_number = con.get(FetcherConfigKey::LIVE_NUMBER).await?;
        }

        Ok(live_number)
    }

    /// 上传蹲饼器配置
    pub async fn upload_cookie_fetcher_configs<'db, D>(
        db: &'db D, configs: Vec<BackFetcherConfig>,
    ) -> LogicResult<()>
    where
        D: GetDatabaseTransaction<Error = DbErr>
            + GetDatabaseConnect<Error = DbErr>
            + 'static,
        D::Transaction<'db>: ConnectionTrait,
        D::Connect<'db>: ConnectionTrait,
    {
        let mut config_in_db_uncheck: Vec<FetcherConfigUncheck> =
            Vec::<FetcherConfigUncheck>::new();

        let mut all_datasources_set = BTreeSet::new();

        // 将上传数据格式换成unchecked结构体
        for BackFetcherConfig { number, server } in configs {
            for (count, Server { groups }) in server.into_iter().enumerate() {
                for Group {
                    name,
                    platform,
                    datasource,
                    interval,
                    interval_by_time_range,
                } in groups
                {
                    for id in datasource {
                        if !all_datasources_set.contains(&id) {
                            all_datasources_set.insert(id);
                        }
                        config_in_db_uncheck.push(FetcherConfigUncheck {
                            live_number: number.require_check(),
                            fetcher_count: (count as i8 + 1).require_check(),
                            group_name: name.clone().require_check(),
                            platform: platform.clone().require_check(),

                            datasource_id: id.require_check(),
                            interval: interval.require_check(),
                            interval_by_time_range: serde_json::to_value(
                                &interval_by_time_range,
                            )?
                            .require_check(),
                        })
                    }
                }
            }
        }
        // 验证传入数据库数据的合法性
        let configs_in_db: Vec<FetcherConfig> =
            FetcherConfigVecChecker::check(
                ((), (), (), (), (), (), ()),
                config_in_db_uncheck,
            )
            .await?;
        if configs_in_db.is_empty() {
            return Ok(());
        }

        // 判断平台是否存在
        let platform = configs_in_db[0].platform.clone();
        if FetcherPlatformConfigSqlOperate::is_platform_exist(
            db.get_connect()?,
            &platform,
        )
        .await?
            && FetcherDatasourceConfigSqlOperate::has_all_datasource_ids(
                db,
                all_datasources_set,
            )
            .await?
        {
            FetcherConfigSqlOperate::create_configs_by_platform(
                db,
                platform,
                configs_in_db,
            )
            .await?;
            // TODO：告诉调度器哪个平台更新了
        } else {
            return Err(LogicError::NoPlatform);
        }

        Ok(())
    }

    /// 获取蹲饼器配置
    pub async fn get_cookie_fetcher_configs<'db, D>(
        db: &'db D, platform: String,
    ) -> LogicResult<Vec<BackFetcherConfig>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let configs_in_db =
            FetcherConfigSqlOperate::find_single_platform_config_list(
                db, platform,
            )
            .await?;

        let mut configs = Vec::<BackFetcherConfig>::new();
        let mut configs_temp =
            HashMap::<i8, HashMap<i8, HashMap<String, Group>>>::new();
        for FetcherConfigModel {
            id: _,
            live_number,
            fetcher_count,
            group_name,
            platform,
            datasource_id,
            interval,
            interval_by_time_range,
        } in configs_in_db
        {
            let server_temp = configs_temp
                .get_mut_or_default(live_number)
                .get_mut_or_default(fetcher_count - 1);

            let group = server_temp
                .get_or_try_create_with::<_, serde_json::Error>(
                    group_name.clone(),
                    || {
                        Ok(Group {
                            name: group_name,
                            platform,
                            datasource: vec![],
                            interval,
                            interval_by_time_range: interval_by_time_range
                                .map(|str| serde_json::from_str(&str))
                                .transpose()?,
                        })
                    },
                )?;
            group.datasource.push(datasource_id);
        }

        // 对configs_temp根据key排序
        let mut configs_temp = Vec::from_iter(configs_temp);
        configs_temp.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        for (live_number, server) in configs_temp.iter() {
            let mut servers_temp = Vec::<Server>::new();
            for i in 1..(*live_number + 1) {
                let mut groups_temp = Vec::<Group>::new();
                if server.get(&(i - 1)).is_some() {
                    // 如果不为空 继续map循环
                    for group in server.get(&(i - 1)).unwrap().values() {
                        groups_temp.push(group.clone());
                    }
                }
                servers_temp.push(Server {
                    groups: groups_temp,
                });
            }
            configs.push(BackFetcherConfig {
                number: *live_number,
                server: servers_temp,
            });
        }

        Ok(configs)
    }
}
