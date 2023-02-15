use std::{
    any::{type_name, TypeId},
    collections::HashMap, sync::Arc,
};

use futures::Future;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

use super::builder::DatabaseBuilder;
use crate::{error::MongoDbError, MongoErr};

mod collection_guard;

pub use collection_guard::CollectionGuard;

/// 构建完成后的结构体
/// 在这种模式下，不允许添加Collection
/// 但是可以通过collection来进行数据操作
pub struct DatabaseManage {
    _db_client: Client,
    collections: Arc<HashMap<TypeId, Collection<()>>>,
}

impl From<DatabaseBuilder> for DatabaseManage {
    fn from(val: DatabaseBuilder) -> Self {
        DatabaseManage {
            collections: Arc::new(val.inner_collect),
            _db_client: val.client,
        }
    }
}

impl DatabaseManage {
    /// 获取一个管理中的Collection ，如果不存在返回 Option::None
    fn collection<C>(&self) -> Option<Collection<C>>
    where
        C: for<'de> Deserialize<'de> + Serialize,
        C: 'static,
        C: Sized,
    {
        let id = std::any::TypeId::of::<C>();

        self.collections
            .get(&id)
            .map(Collection::clone_with_type::<C>)
    }

    /// 对完成获取的数据库进行数据操作
    /// - handle 为一个异步操作闭包
    /// 形如 `async fn function(&Collection<C>)->Result<O, E>`
    ///
    /// - 这里要求 E 允许 `MongodbError` 可以转换为E
    ///
    /// - 函数通过泛型参数自动识别并寻找对应的Collection
    /// 如果Collection 未被创建，就会允许失败
    pub async fn doing<F, C, Fut, O>(
        &self, handle: F,
    ) -> Result<O, MongoDbError>
    where
        C: for<'de> serde::Deserialize<'de>
            + 'static
            + Sized
            + serde::Serialize,
        F: FnOnce(Collection<C>) -> Fut,
        Fut: Future<Output = Result<O, MongoErr>>,
    {
        let collection = self.collection::<C>().ok_or_else(|| {
            MongoDbError::CollectionNotFound(type_name::<C>())
        })?;
        handle(collection).await.map_err(MongoDbError::from)
    }

    pub fn get_collection<C>(
        &self,
    ) -> Result<CollectionGuard<C>, MongoDbError>
    where
        C: for<'de> serde::Deserialize<'de> + 'static + serde::Serialize,
    {
        self.collection::<C>()
            .ok_or_else(|| MongoDbError::CollectionNotFound(type_name::<C>()))
            .map(|c| CollectionGuard { inner: c })
    }
}
