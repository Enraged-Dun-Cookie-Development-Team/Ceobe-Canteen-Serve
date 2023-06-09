mod admin;
mod ceobe_cookie_search_content;
mod ceobe_operation_announcement;
mod ceobe_operation_app_version;
mod ceobe_operation_resource;
mod ceobe_operation_video;
mod fetcher_config;
mod fetcher_datasource_combination;
mod fetcher_datasource_config;
mod fetcher_global_config;
mod fetcher_platform_config;

pub use sea_orm_migration::MigratorTrait;
use sea_orm_migration::{async_trait, MigrationTrait};
pub use sql_models::{self, sql_connection};
macro_rules! migrate_group {
    [$($t:expr)*] => {
        vec![
            $(
                Box::new($t)
            ),*
            ]
        };
    }
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        migrate_group![
            admin::m20220324_222712_create_user::Migration
            admin::m20220429_142528_alter_user::Migration
            admin::m20220429_230336_alter_user::Migration
            admin::m20220722_082735_change_user_table_name::Migration
            ceobe_operation_announcement::m20220724_115553_create::Migration
            ceobe_operation_video::m20220724_090218_new_video_model::Migration
            ceobe_operation_app_version::m20220803_104932_create::Migration
            ceobe_operation_resource::m20220809_160731_create::Migration
            fetcher_platform_config::m20221217_155027_create::Migration
            fetcher_datasource_config::m20221217_155038_create::Migration
            fetcher_global_config::m20221217_155049_create::Migration
            fetcher_config::m20221217_155140_create::Migration
            admin::m20221218_001732_charset_and_collate::Migration
            ceobe_operation_app_version::m20221218_001756_charset_and_collate::Migration
            ceobe_operation_resource::m20221218_001918_charset_and_collate::Migration
            ceobe_operation_video::m20221218_001928_charset_and_collate::Migration
            ceobe_operation_announcement::m20221218_002839_charset_and_collate::Migration
            fetcher_config::m20221231_150609_alter_interval::Migration
            fetcher_platform_config::m20221231_150609_alter_min_interval::Migration
            fetcher_datasource_config::m20221231_200206_alter_nickname::Migration
            fetcher_config::m20221231_205557_alter_group_name::Migration
            fetcher_config::m20230101_013601_create_index::Migration
            fetcher_datasource_config::m20230217_135012_add_sort_detele::Migration
            fetcher_datasource_combination::m20230310_213209_create::Migration
            ceobe_operation_resource::m20230422_150425_add_type::Migration
            fetcher_datasource_config::m20230528_110010_add_jump_url::Migration
            ceobe_cookie_search_content::m20230606_135941_create::Migration
        ]
    }
}
