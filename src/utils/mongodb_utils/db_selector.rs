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
    mongo_manager::MongoManager,
};

pub trait DbSelector {
    fn db_name() -> &'static str;
}

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
struct MongoDbSelector<S> {
    db: DbManager,
    _phantom: PhantomData<S>,
}

impl<S> MongoDbSelector<S> {
    pub fn doing<F, C>(&self, handle: F) -> Result<(), MongoDbError>
    where
        C: for<'de> serde::Deserialize<'de> + 'static + Sized + serde::Serialize,
        F: FnOnce(&Collection<C>),
    {
        let collection = self
            .db
            .collection::<C>()
            .ok_or(MongoDatabaseCollectionNotFound)?;
        handle(&collection);

        Ok(())
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
