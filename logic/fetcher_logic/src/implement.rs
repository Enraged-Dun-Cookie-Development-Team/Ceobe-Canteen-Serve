use crate::error::LogicResult;
use serde_json::{Map, Value};
use sql_models::{
    fetcher::global_config::{
        checkers::global_config_data::FetcherGlobalConfig,
        models::model_global_config::Model,
        operate::FetcherGlobalConfigSqlOperate,
    },
    sql_connection::{
        database_traits::get_connect::GetDatabaseConnect,
        sea_orm::{ConnectionTrait, DbErr},
    },
};

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
    let vec = config
        .into_iter()
        .map(|(key, value)| FetcherGlobalConfig {
                key: key.to_string(),
                value: value.to_string(),
            })
            .collect();
    FetcherGlobalConfigSqlOperate::create_or_update(db, vec).await?;
    Ok(())
}
