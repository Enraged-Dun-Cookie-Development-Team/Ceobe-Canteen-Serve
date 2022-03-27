//! Database Selector 数据库选择器
//! 通常情况下，mongodb 可以对不同的子应用使用不同的数据库
//! 有些情况下，我们希望能在获取参数时直接得到数据库选择器

use std::marker::PhantomData;

use actix_web::web::Data;
use futures::Future;
use mongodb::Collection;

use crate::utils::req_pretreatment::Pretreatment;

use super::{
    db_manager::DbManager,
    error::{MongoDatabaseCollectionNotFound, MongoDatabaseNotFound, MongoDbError},
    mongo_manager::MongoManager, MongoErr,
};

/// 数据库选择器trait
/// 提供数据名称来选择当前使用的数据库
pub trait DbSelector {
    /// 用于获取数据库名称的接口
    fn db_name() -> &'static str;
}

/// 便捷构造数据库选择器的macro
/// ## example
/// ```rust
/// db_selector!(
///     pub           // 可见性控制符号
///     Mansion="mansion-db"
///     // |        |----- 要选择的数据库名称
///     // |-------- 新建的选择器名称
///     );
/// ```
#[macro_export]
macro_rules! db_selector {
    ($v:vis $name:ident=$l:literal) => {
        $v struct $name;

        impl $crate::utils::mongodb_utils::db_selector::DbSelector for $name{
            fn db_name()->& 'static str{
                $l
            }
        }
    };
}

/// mongo database Selector
/// mongo 数据库选择器，根据给定的 `S`
/// 在编译期完成数据库选择
pub struct MongoDbSelector<S> {
    db: DbManager,
    _phantom: PhantomData<S>,
}

impl<S> MongoDbSelector<S> {
    /// 对完成获取的数据库进行数据操作
    /// - handle 为一个异步操作闭包
    /// 形如 `async fn function(&Collection<C>)->Result<O, E>`
    /// 
    /// - 这里要求 E 允许 `MongodbError` 可以转换为E
    /// 
    /// - 函数通过泛型参数自动识别并寻找对应的Collection
    /// 如果Collection 未被创建，就会允许失败
    pub async fn doing<F, C, Fut,E, O>(&self, handle: F) -> Result<O, E>
    where
        C: for<'de> serde::Deserialize<'de> + 'static + Sized + serde::Serialize,
        F: FnOnce(Collection<C>) -> Fut,
        Fut: Future<Output = Result<O, MongoErr>>,
        E: From<MongoDbError>,
    {
        let collection = self
            .db
            .collection::<C>()
            .ok_or(MongoDatabaseCollectionNotFound)
            .map_err(MongoDbError::from)?;
        handle(collection).await.map_err(MongoDbError::from).map_err(E::from)
    }
}

impl<S: DbSelector> Pretreatment for MongoDbSelector<S> {
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = Self;

    type Err = MongoDbError;

    fn call<'db>(req: &'db actix_web::HttpRequest, _: &'db mut actix_http::Payload) -> Self::Fut {
        let mongo = req
            .app_data::<Data<MongoManager>>()
            .expect("MongoDb 数据库未找到");
        let db = mongo
            .get_db(S::db_name())
            .ok_or(MongoDatabaseNotFound)
            .cloned();
        async move {
            Ok(Self {
                db: db?,
                _phantom: Default::default(),
            })
        }
    }
}
