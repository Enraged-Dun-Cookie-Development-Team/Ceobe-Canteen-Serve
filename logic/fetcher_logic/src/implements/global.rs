use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, LiteChecker,
    ToCheckRequire,
};
use persistence::{
    fetcher::{
        global_config::ToGlobal,
        models::global_config::{
            checkers::global_config_data::{
                FetcherGlobalConfigChecker, FetcherGlobalConfigUncheck,
            },
            models::model_global_config::Model,
        },
        ToFetcher,
    },
    mysql::SqlDatabaseOperate,
};
use serde_json::{Map, Value};

use super::FetcherConfigLogic;
use crate::error::LogicResult;

impl FetcherConfigLogic {
    /// 从数据库获取json的key和value，拼接成json格式返回
    pub async fn get_global_configs(
        db: SqlDatabaseOperate,
    ) -> LogicResult<Value> {
        // 获取数据库configs：Vec<Model>
        let global_config_kv = db.fetcher().global().get_all().await?;

        let mut map = Map::with_capacity(global_config_kv.len());
        // 转成map格式
        for Model { key, value, .. } in global_config_kv {
            map.insert(key, serde_json::from_str(&value)?);
        }
        // 转成json格式
        Ok(Value::Object(map))
    }

    /// 接收来自controller的json格式
    pub async fn set_global_config(
        db: SqlDatabaseOperate, config: Map<String, Value>,
    ) -> LogicResult<()> {
        // 迭代map将<Key, Value>转Vec<{key, value}>， 并将value转字符串
        let vec = config.into_iter().map(|(key, value)| {
            FetcherGlobalConfigUncheck {
                key: key.require_check(),
                value: value.to_string().require_check(),
            }
        });
        // 验证传入数据库数据的合法性
        let configs =
            IntoIterChecker::<_,FetcherGlobalConfigChecker,Vec<_>>::lite_check(vec).await?;
        db.fetcher().global().create_or_update(configs).await?;
        Ok(())
    }
}
