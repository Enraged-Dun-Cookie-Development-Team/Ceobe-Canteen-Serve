mod admin;

pub use sea_orm_migration::MigratorTrait;
use sea_orm_migration::{async_trait, MigrationTrait};
pub use sql_models::{self, sql_connection};
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        crate::migrate_group![
            admin::m20220324_222712_create_user::Migration
            admin::m20220429_142528_alter_user::Migration
            admin::m20220429_230336_alter_user::Migration
        ]
    }
}
#[macro_export(crate)]
macro_rules! migrate_group {
    [$($t:expr)*] => {
        vec![
            $(
                Box::new($t)
            ),*
        ]
    };
}
