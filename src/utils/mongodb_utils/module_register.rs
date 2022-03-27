use std::marker::PhantomData;

use futures::Future;

use super::{db_manager::DbBuild, db_selector::DbSelector};
/// 数据库Collection 注册器
/// 挂载在Controller 中在提供给外部
pub struct ModuleRegister<S, L> {
    _select: PhantomData<S>,
    loader: L,
}

pub trait MongoRegister {
    fn db_name(&self) -> &'static str;
    type Fut: Future<Output = DbBuild>;
    fn register(self, db: DbBuild) -> Self::Fut;
}

/// Collection 加载器，向 DbBuild 中添加Collection
pub trait CollectionLoader {
    type Fut: Future<Output = DbBuild>;
    fn loader(self, db: DbBuild) -> Self::Fut;
}

impl<S: DbSelector, L: CollectionLoader> MongoRegister for ModuleRegister<S, L> {
    fn db_name(&self) -> &'static str {
        S::db_name()
    }

    fn register(self, db: DbBuild) -> Self::Fut {
        self.loader.loader(db)
    }

    type Fut = L::Fut;
}
impl<F, Fut> CollectionLoader for F
where
    F: FnOnce(DbBuild) -> Fut,
    Fut: Future<Output = DbBuild>,
{
    type Fut = Fut;

    fn loader(self, db: DbBuild) -> Self::Fut {
        self(db)
    }
}

impl<S, L> ModuleRegister<S, L> {
    /// 创建新的加载器
    pub fn new(loader: L) -> Self
    where
        L: CollectionLoader,
        S: DbSelector,
    {
        Self {
            _select: PhantomData,
            loader,
        }
    }
}
#[macro_export]
macro_rules! generate_collection_register {
    {$selector:ty => $loader:expr} => {
        $crate::utils::mongodb_utils::module_register::ModuleRegister::<$selector,_>::new($loader)
    };
}
