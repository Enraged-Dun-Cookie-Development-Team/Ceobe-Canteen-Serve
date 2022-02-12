use sea_schema::migration::{
    sea_query::{self, *},
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new_with_type(Users::Name, ColumnType::String(Some(16)))
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new_with_type(Users::Pwd, ColumnType::Char(Some(64))).not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Policy)
                            .enumeration("Policy", ["ceobe-user", "admin", "mansion-uploader"])
                            .not_null()
                    )
                    .to_owned(),
            )
            .await?;

        manager.create_index(sea_query::index::Index::create()
            .name("policy-idx")
            .table(Users::Table)
            .col(Users::Policy)
            .to_owned()
    ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            sea_query::Table::drop()
            .table(Users::Table)
            .to_owned()
        ).await
    }
}

#[derive(Iden)]
pub enum Users {
    Table,

    Id,

    Name,
    Pwd,

    Policy,
}
