
use sea_orm_migration::prelude::*;
use sql_models::ceobe_operation::resource::countdown::CountdownType;
use super::{m20220809_160731_create::CeobeOperationResource};


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20230422_150425_ceobe_operation_resource_add_type" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(CeobeOperationResource::Table).add_column(
            &mut ColumnDef::new_with_type(CeobeOperationResource::CountdownType, CountdownType::column_type())
        );
        manager.alter_table(al).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut al = sea_query::Table::alter();
        al.table(CeobeOperationResource::Table).drop_column(CeobeOperationResource::CountdownType);
        manager.alter_table(al).await?;

        Ok(())
    }
}
