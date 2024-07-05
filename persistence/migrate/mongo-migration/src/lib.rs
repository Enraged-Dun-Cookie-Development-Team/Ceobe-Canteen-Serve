use async_trait::async_trait;
use mongo_migrate_util::{Manager, MigratorTrait};
use mongodb::error::Error;

mod migrations;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    async fn migrating(&self, manage: &mut Manager<'_>) -> Result<(), Error> {
        manage
            .append(migrations::bakery::mansion::Migration)
            .await?
            .append(migrations::ceobe::operation::plugin_version::Migration)
            .await?
            .append(migrations::ceobe::user::property::Migration)
            .await?
            .append(migrations::ceobe::cookie::temp_list::Migration)
            .await?
            .append(migrations::ceobe::cookie::analyze::Migration)
            .await?
            .append(migrations::ceobe::cookie::raw::Migration)
            .await?
            .append(migrations::ceobe::cookie::terra_comic::Migration)
            .await?
            .append(migrations::ceobe::operation::release_version::Migration)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test{
    use serde::Deserialize;
    use database_traits::initial::connect_db_with_migrate;
    use mongo_connection::{DatabaseManage, DbConnectConfig};
    use crate::Migrator;
    
    #[tokio::test]
    async fn test_migrate(){
        #[derive(Deserialize)]
        pub struct MongoDbConfig; 
        impl DbConnectConfig for MongoDbConfig{
            fn scheme(&self) -> &str {
                "mongodb"
            }

            fn username(&self) -> &str {
                "ceobe"
            }

            fn password(&self) -> &str {
                "114514"
            }

            fn host(&self) -> &str {
                "localhost"
            }

            fn port(&self) -> u16 {
                27017
            }

            fn name(&self) -> &str {
                "ceobe_canteen"
            }
        }
        
       let _ =  connect_db_with_migrate::<DatabaseManage,_,_>(
            &MongoDbConfig,
            Migrator
        ).await.expect("Migrate error");
    }
}