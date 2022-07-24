mod admin;
mod video;

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
            admin::m20220722_082735_change_user_table_name::Migration
            video::m20220724_090218_new_video_model::Migration
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
