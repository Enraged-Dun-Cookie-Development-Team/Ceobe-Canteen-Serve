use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, LiteChecker,
    ToCheckRequire,
};
use serde_json::{Map, Value};
use sql_models::{
    fetcher::global_config::{
        checkers::global_config_data::{
            FetcherGlobalConfigChecker, FetcherGlobalConfigUncheck,
        },
        models::model_global_config::Model,
        operate::FetcherGlobalConfigSqlOperate,
    },
    sql_connection::{
        database_traits::get_connect::GetDatabaseConnect,
        sea_orm::{ConnectionTrait, DbErr},
    },
};

use super::GlobalConfig;
use crate::error::LogicResult;

impl GlobalConfig {
    /// 从数据库获取json的key和value，拼接成json格式返回
    pub async fn get_all<'db, D>(&self, db: &'db D) -> LogicResult<Value>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        // 获取数据库configs：Vec<Model>
        let global_config_kv =
            FetcherGlobalConfigSqlOperate::get_all(db).await?;

        let mut map = Map::with_capacity(global_config_kv.len());
        // 转成map格式
        for Model { key, value, .. } in global_config_kv {
            map.insert(key, serde_json::from_str(&value)?);
        }
        // 转成json格式
        Ok(Value::Object(map))
    }

    /// 接收来自controller的json格式
    pub async fn set<'db, D>(
        &self, db: &'db D, config: Map<String, Value>,
    ) -> LogicResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
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
        FetcherGlobalConfigSqlOperate::create_or_update(db, configs).await?;
        Ok(())
    }
}
