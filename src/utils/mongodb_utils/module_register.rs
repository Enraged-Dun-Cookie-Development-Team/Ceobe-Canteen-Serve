use super::{db_manager::DbBuild, db_selector::DbSelector};

pub trait ModuleRegister: DbSelector {
    fn register(db: &mut DbBuild);
}
