//! Database Manage Mongo数据库单库管理
//!
//! 对Mongo数据库的单个数据库进行管理
//! 在mongodb中，可能会存在多个数据库
//! 这个模块提供的控制多个数据库的可能
use std::{any::TypeId, collections::HashMap, sync::Arc};

use mongodb::Collection;
use serde::{Deserialize, Serialize};

use super::MongoDb;

/// 启动时构造数据库信息的类型
/// 可以添加并进行Collection的配置
pub struct DbBuild {
    db: MongoDb,
    inner_collect: HashMap<TypeId, Collection<()>>,
}

/// 构建完成后的结构体
/// 在这种模式下，不允许添加Collection
/// 但是可以通过collection来进行数据操作
pub struct DbManager {
    inner_collect: Arc<HashMap<TypeId, Collection<()>>>,
}

impl Clone for DbManager {
    fn clone(&self) -> Self {
        Self {
            inner_collect: self.inner_collect.clone(),
        }
    }
}

impl DbBuild {
    pub(super) fn new(db: MongoDb) -> Self {
        Self {
            db,
            inner_collect: HashMap::default(),
        }
    }
}

impl mongo_migration::utils::migrator::DbManager for DbBuild {
    fn get_db(self) -> mongodb::Database { self.db }

    fn from_collects<
        I: IntoIterator<Item = (std::any::TypeId, mongodb::Collection<()>)>,
    >(
        db: mongodb::Database, iter: I,
    ) -> Self {
        Self {
            db,
            inner_collect: iter.into_iter().collect(),
        }
    }
}

impl Into<DbManager> for DbBuild {
    fn into(self) -> DbManager {
        DbManager {
            inner_collect: Arc::new(self.inner_collect),
        }
    }
}

impl DbManager {
    /// 获取一个管理中的Collection ，如果不存在返回 Option::None
    pub fn collection<C>(&self) -> Option<Collection<C>>
    where
        C: for<'de> Deserialize<'de> + Serialize,
        C: 'static,
        C: Sized,
    {
        let id = std::any::TypeId::of::<C>();

        let collect = self.inner_collect.get(&id)?.clone_with_type::<C>();

        Some(collect)
    }
}
