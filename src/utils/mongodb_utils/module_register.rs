use futures::Future;

use super::db_manager::DbBuild;
/// 数据库Collection 注册器
/// 挂载在Controller 中在提供给外部
pub struct ModuleRegister<L> {
    loader: L,
}

pub trait MongoRegister {
    type Fut: Future<Output = DbBuild>;
    fn register(self, db: DbBuild) -> Self::Fut;
}

/// Collection 加载器，向 DbBuild 中添加Collection
pub trait CollectionLoader {
    type Fut: Future<Output = DbBuild>;
    fn loader(self, db: DbBuild) -> Self::Fut;
}

impl<L: CollectionLoader> MongoRegister for ModuleRegister<L> {
    type Fut = L::Fut;

    fn register(self, db: DbBuild) -> Self::Fut { self.loader.loader(db) }
}
impl<F, Fut> CollectionLoader for F
where
    F: FnOnce(DbBuild) -> Fut,
    Fut: Future<Output = DbBuild>,
{
    type Fut = Fut;

    fn loader(self, db: DbBuild) -> Self::Fut { self(db) }
}

impl<L> ModuleRegister<L> {
    /// 创建新的加载器
    pub fn new(loader: L) -> Self
    where
        L: CollectionLoader,
    {
        Self { loader }
    }
}
#[macro_export]
macro_rules! generate_collection_register {
    { $loader:expr} => {
        $crate::utils::mongodb_utils::module_register::ModuleRegister::new($loader)
    };
}
