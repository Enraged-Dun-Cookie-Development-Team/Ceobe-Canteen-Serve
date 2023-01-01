use std::collections::{BTreeSet, HashMap};

use checker::{CheckRequire, Checker};
use page_size::request::PageSize;
use redis::{AsyncCommands, RedisError};
use redis_global::redis_key;
use serde_json::{Map, Value};
use sql_models::{
    fetcher::{
        config::{
            checkers::config_data::{
                FetcherConfig, FetcherConfigUncheck, FetcherConfigVecChecker,
            },
            models::model_config::Model as FetcherConfigModel,
            operate::FetcherConfigSqlOperate,
        },
        datasource_config::{
            checkers::datasource_config_data::FetcherDatasourceConfig,
            operate::FetcherDatasourceConfigSqlOperate,
        },
        global_config::{
            checkers::global_config_data::{
                FetcherGlobalConfigUncheck, FetcherGlobalConfigVecChecker,
            },
            models::model_global_config::Model,
            operate::FetcherGlobalConfigSqlOperate,
        },
        platform_config::{
            models::model_platform_config::PlatformWithHasDatasource,
            operate::FetcherPlatformConfigSqlOperate,
        },
    },
    sql_connection::{
        database_traits::get_connect::{
            GetDatabaseConnect, GetDatabaseTransaction,
            GetMutDatabaseConnect, TransactionOps,
        },
        sea_orm::{ConnectionTrait, DbErr},
    },
};

use crate::{
    error::{LogicError, LogicResult},
    view::{BackFetcherConfig, Group, Server},
};

// 分页获取获取平台信息并且附带该平台下有无数据源
pub async fn get_platform_list_with_has_datasource<'db, D>(
    db: &'db D, page_size: PageSize,
) -> LogicResult<Vec<PlatformWithHasDatasource>>
where
    D: GetDatabaseConnect<Error = DbErr> + 'static,
    D::Connect<'db>: ConnectionTrait,
{
    // 分页查询平台列表
    let platform_list =
        FetcherPlatformConfigSqlOperate::find_platform_list_by_page_size(
            db, page_size,
        )
        .await?;
    // 获取平台的type的数组
    let platforms = platform_list
        .iter()
        .map(|platform_item| platform_item.type_id.clone())
        .collect();
    // 查询哪些平台下有数据源
    let platform_datasource_exist_map =
        FetcherDatasourceConfigSqlOperate::has_datasource_from_platforms(
            db, platforms,
        )
        .await?;

    let resp = platform_list
        .into_iter()
        .map(|platform_item| {
            PlatformWithHasDatasource {
                id: platform_item.id,
                type_id: platform_item.type_id.clone(),
                platform_name: platform_item.platform_name,
                min_request_interval: platform_item.min_request_interval,
                has_datasource: platform_datasource_exist_map
                    .contains(&platform_item.type_id),
            }
        })
        .collect();

    Ok(resp)
}

// 新建数据源配置
pub async fn create_datasource_config<'db, D>(
    db: &'db D, datasource_config: FetcherDatasourceConfig,
) -> LogicResult<()>
where
    D: GetDatabaseConnect<Error = DbErr> + 'static,
    D::Connect<'db>: ConnectionTrait,
{
    // 验证平台存在
    if FetcherPlatformConfigSqlOperate::is_platform_exist_with_raw_db(
        db,
        &datasource_config.platform,
    )
    .await?
    {
        // 创建数据源
        FetcherDatasourceConfigSqlOperate::create_database_config(
            db,
            datasource_config,
        )
        .await?;
    }
    else {
        return Err(LogicError::NoPlatform);
    }
    Ok(())
}

// 从数据库获取json的key和value，拼接成json格式返回
pub async fn get_global_configs<'db, D>(db: &'db D) -> LogicResult<Value>
where
    D: GetDatabaseConnect<Error = DbErr> + 'static,
    D::Connect<'db>: ConnectionTrait,
{
    // 获取数据库configs：Vec<Model>
    let global_config_kv =
        FetcherGlobalConfigSqlOperate::get_all_global_configs(db).await?;

    let mut map = Map::new();
    // 转成map格式
    for Model { key, value, id: _ } in global_config_kv.into_iter() {
        map.insert(key, serde_json::from_str(&value)?);
    }
    // 转成json格式
    let obj = Value::Object(map);
    Ok(obj)
}

// 接收来自controller的json格式
pub async fn set_global_config<'db, D>(
    db: &'db D, config: Map<String, Value>,
) -> LogicResult<()>
where
    D: GetDatabaseConnect<Error = DbErr> + 'static,
    D::Connect<'db>: ConnectionTrait,
{
    // 迭代map将<Key, Value>转Vec<{key, value}>， 并将value转字符串
    let vec: Vec<FetcherGlobalConfigUncheck> = config
        .into_iter()
        .map(|(key, value)| {
            FetcherGlobalConfigUncheck {
                key: CheckRequire::new_with_no_checker(key),
                value: CheckRequire::new_with_no_checker(value.to_string()),
            }
        })
        .collect();
    // 验证传入数据库数据的合法性
    let configs = FetcherGlobalConfigVecChecker::check(((), ()), vec).await?;
    FetcherGlobalConfigSqlOperate::create_or_update(db, configs).await?;
    Ok(())
}

// 删除一个数据源
pub async fn delete_datasource_by_id<'db, D>(
    db: &'db D, id: i32,
) -> LogicResult<()>
where
    D: GetDatabaseConnect<Error = DbErr> + GetDatabaseTransaction + 'db,
    D::Transaction<'db>: ConnectionTrait,
{
    // 开事务
    let ctx = db.get_transaction().await?;

    // 删除蹲饼器配置中的所有有datasource_id的配置
    FetcherConfigSqlOperate::delete_fetcher_configs_by_datasource_id(
        &ctx, id,
    )
    .await?;
    // 删除数据源
    FetcherDatasourceConfigSqlOperate::delete_one_datasource_config(&ctx, id)
        .await?;

    // 提交事务
    ctx.submit().await?;

    // TODO：告诉调度器哪个平台更新了

    Ok(())
}

// 获取蹲饼器最大存活数量
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
    if con
        .exists(redis_key::fetcher::COOKIE_FETCHER_CONFIG_LIVE_NUMBER)
        .await?
    {
        // 获取key的值
        live_number = con
            .get(redis_key::fetcher::COOKIE_FETCHER_CONFIG_LIVE_NUMBER)
            .await?;
    }

    Ok(live_number)
}

// 上传蹲饼器配置
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
                        live_number: CheckRequire::new_with_no_checker(
                            number,
                        ),
                        fetcher_count: CheckRequire::new_with_no_checker(
                            count as i8 + 1,
                        ),
                        group_name: CheckRequire::new_with_no_checker(
                            name.clone(),
                        ),
                        platform: CheckRequire::new_with_no_checker(
                            platform.clone(),
                        ),
                        datasource_id: CheckRequire::new_with_no_checker(id),
                        interval: CheckRequire::new_with_no_checker(interval),
                        interval_by_time_range:
                            CheckRequire::new_with_no_checker(
                                serde_json::to_value(
                                    &interval_by_time_range,
                                )?,
                            ),
                    })
                }
            }
        }
    }
    // 验证传入数据库数据的合法性
    let configs_in_db: Vec<FetcherConfig> = FetcherConfigVecChecker::check(
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
    }
    else {
        return Err(LogicError::NoPlatform);
    }

    Ok(())
}

// 获取蹲饼器配置
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
        if configs_temp.get(&live_number).is_none() {
            configs_temp.insert(
                live_number,
                HashMap::<i8, HashMap<String, Group>>::new(),
            );
        }
        if configs_temp
            .get(&live_number)
            .unwrap()
            .get(&(fetcher_count - 1))
            .is_none()
        {
            configs_temp
                .get_mut(&live_number)
                .unwrap()
                .insert(fetcher_count - 1, HashMap::<String, Group>::new());
        }
        if configs_temp
            .get(&live_number)
            .unwrap()
            .get(&(fetcher_count - 1))
            .unwrap()
            .get(&group_name)
            .is_none()
        {
            configs_temp
                .get_mut(&live_number)
                .unwrap()
                .get_mut(&(fetcher_count - 1))
                .unwrap()
                .insert(
                    group_name.clone(),
                    Group {
                        name: group_name,
                        platform,
                        datasource: vec![datasource_id],
                        interval,
                        interval_by_time_range: match interval_by_time_range {
                            Some(str) => serde_json::from_str(&str)?,
                            None => None,
                        },
                    },
                );
        }
        else if let Some(group) = configs_temp
            .get_mut(&live_number)
            .unwrap()
            .get_mut(&(fetcher_count - 1))
            .unwrap()
            .get_mut(&group_name)
        {
            group.datasource.push(datasource_id);
        }
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
