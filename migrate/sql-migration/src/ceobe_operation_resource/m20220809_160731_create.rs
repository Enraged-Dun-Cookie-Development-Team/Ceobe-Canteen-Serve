use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220809_160731_ceobe_operation_resource_create"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::create();
        table
            .table(CeobeOperationResource::Table)
            .if_not_exists()
            .col(ColumnDef::new(Id).integer().auto_increment().primary_key())
            .col(
                ColumnDef::new(Ty)
                    .enumeration(
                        ResourceType::Name,
                        [ResourceType::AllAvailable, ResourceType::Countdown], // ["resource_all_available", "countdown"],
                    )
                    .not_null()
                    .default("countdown"),
            )
            .col(
                ColumnDef::new(Message)
                    .string_len(255)
                    .default("")
                    .not_null(),
            )
            .col(
                ColumnDef::new(BannerInfo)
                    .string_len(255)
                    .default("")
                    .not_null(),
            )
            .col(
                ColumnDef::new(CountdownEnd)
                    .date_time()
                    .not_null()
                    .default(get_zero_data_time()),
            )
            .col(ColumnDef::new(StartTime).date_time().not_null())
            .col(ColumnDef::new(OverTime).date_time().not_null())
            // soft remove
            .col(ColumnDef::new(CreateAt).date_time().not_null())
            .col(ColumnDef::new(ModifyAt).date_time().not_null())
            .col(
                ColumnDef::new(DeleteAt)
                    .date_time()
                    .not_null()
                    .default(get_zero_data_time()),
            );
        manager.create_table(table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = sea_query::Table::drop();
        table.table(DbTable);
        manager.drop_table(table).await?;

        Ok(())
    }
}

use sql_models::get_zero_data_time;
use CeobeOperationResource::{Table as DbTable, *};

#[derive(Debug, Iden)]
enum CeobeOperationResource {
    Table,
    Id,
    // type
    Ty,
    Message,
    BannerInfo,
    CountdownEnd,
    StartTime,
    OverTime,

    // soft remove
    CreateAt,
    ModifyAt,
    DeleteAt,
}

#[derive(Debug, Iden)]
enum ResourceType {
    #[iden(rename = "resource_type")]
    Name,
    #[iden(rename = "resource_all_available")]
    AllAvailable,
    #[iden(rename = "countdown")]
    Countdown,
}

#[cfg(test)]
mod test {
    use sea_orm_migration::prelude::Iden;

    use super::ResourceType;
    #[test]
    fn test() {
        println!("{:?}", ResourceType::Name.to_string());
        println!("{:?}", ResourceType::AllAvailable.to_string());
        println!("{:?}", ResourceType::Countdown.to_string());
    }
}
