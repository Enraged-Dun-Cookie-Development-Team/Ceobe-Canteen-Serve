mod admin;

pub use sea_schema::migration::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        crate::migrate_group![
            admin::m20220324_222712_create_user::Migration
            admin::m20220429_142528_alter_user::Migration
        ]
    }
}
#[macro_export]
macro_rules! migrate_group {
    [$($t:expr)*] => {
        vec![
            $(
                Box::new($t)
            ),*
        ]
    };
}
