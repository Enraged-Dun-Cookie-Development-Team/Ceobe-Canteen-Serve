
use sea_orm_migration::{prelude::*, sea_orm::{Statement, ConnectionTrait}};


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20221218_001756_ceobe_operation_app_version_charset_and_collate" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE ceobe_operation_app_version CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
        "#;
        let stmt = Statement::from_string(
            manager.get_database_backend(),
            sql.to_owned(),
        );

        manager.get_connection().execute(stmt).await?;
        Ok(())    
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
