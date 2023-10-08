use sea_orm_migration::{prelude::*, sea_orm::{Statement, ConnectionTrait}};

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230729_211229_ceobe_operation_desktop_version_create"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("m20230729_211229_create/up.sql");

        let stmt = Statement::from_string(
            manager.get_database_backend(),
            sql.to_owned(),
        );

        manager.get_connection().execute(stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("m20230729_211229_create/down.sql");

        let stmt = Statement::from_string(
            manager.get_database_backend(),
            sql.to_owned(),
        );

        manager.get_connection().execute(stmt).await?;
        Ok(())
    }
}
