use std::marker::PhantomData;

use super::{db_manager::DbBuild, db_selector::DbSelector};
/// 数据库Collection 注册器
/// 挂载在Controller 中在提供给外部
pub struct ModuleRegister<S, L> {
    _select: PhantomData<S>,
    loader: L,
}


pub trait MongoRegister{
    fn db_name(&self) -> &'static str;
    fn register(self, db: &mut DbBuild);
}

/// Collection 加载器，向 DbBuild 中添加Collection
pub trait CollectionLoader {
    fn loader(self, db: &mut DbBuild);
}

impl<S:DbSelector, L:CollectionLoader> MongoRegister for ModuleRegister<S, L> {
    fn db_name(&self) -> &'static str {
        S::db_name()
    }

    fn register(self, db: &mut DbBuild) {
        self.loader.loader(db)
    }
}
impl<F> CollectionLoader for F
where
    F: FnOnce(&mut DbBuild),
{
    fn loader(self, db: &mut DbBuild) {
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
