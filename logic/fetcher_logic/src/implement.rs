use crate::error::{LogicResult, LogicError};
use checker::{Checker, CheckRequire};
use checker::prefabs::no_check::NoCheck;
use range_limit::RangeBoundLimit;
use range_limit::limits::max_limit::MaxLimit;
use serde_json::{Map, Value};
use sql_models::fetcher::config::operate::FetcherConfigSqlOperate;
use sql_models::fetcher::datasource_config::checkers::datasource_config_data::FetcherDatasourceConfig;
use sql_models::fetcher::datasource_config::operate::FetcherDatasourceConfigSqlOperate;
use sql_models::fetcher::global_config::checkers::global_config_data::{FetcherGlobalConfigUncheck, FetcherGlobalConfigVecChecker};
use sql_models::fetcher::platform_config::models::model_platform_config::PlatformWithHasDatasource;
use sql_models::fetcher::platform_config::operate::FetcherPlatformConfigSqlOperate;
use sql_models::sql_connection::database_traits::get_connect::{GetDatabaseTransaction, TransactionOps};
use sql_models::{
    fetcher::global_config::{
        checkers::global_config_data::{
            FetcherGlobalConfig, FetcherGlobalConfigChecker,
        },
        models::model_global_config::Model,
        operate::FetcherGlobalConfigSqlOperate,
    },
    sql_connection::{
        database_traits::get_connect::GetDatabaseConnect,
        sea_orm::{ConnectionTrait, DbErr},
    },
};
use page_size::request::PageSize;

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
    let platforms = platform_list
        .iter()
        .map(|platform_item| platform_item.type_id.clone())
        .collect();
    // 查询map
    let platform_datasource_exist_map =
        FetcherDatasourceConfigSqlOperate::has_datasource_from_platforms(
            db, platforms,
        )
        .await?;

    let resp = platform_list
        .into_iter()
        .map(|platform_item| PlatformWithHasDatasource {
            id: platform_item.id,
            type_id: platform_item.type_id.clone(),
            platform_name: platform_item.platform_name,
            min_request_interval: platform_item.min_request_interval,
            has_datasource: platform_datasource_exist_map
                .contains_key(&platform_item.type_id),
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
    } else {
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
        map.insert(key, Value::String(value));
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
        .map(|(key, value)| FetcherGlobalConfigUncheck {
            key: CheckRequire::new_with_no_checker(key),
            value: CheckRequire::new_with_no_checker(value.to_string()),
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
