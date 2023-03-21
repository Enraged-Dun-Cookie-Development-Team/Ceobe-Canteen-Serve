use axum_starter::{prepare, PrepareStateEffect};
use fetcher::{datasource_config::ToDatasource, ToFetcher};
use general_request_client::client::RequestClient;
use mob_push::{MobPushConfig, MobPusher};
use mongo_models::mongo_connection::MongoDatabaseOperate;
use sql_models::sql_connection::SqlDatabaseOperate;
use tokio::sync::mpsc::Sender;

use crate::push_client::ReqClient;

use super::{
    error::InternalError, push_entity::CookiePushEntity,
    push_manager::PushManager,
};

pub struct InitManage;

impl PrepareStateEffect for InitManage {
    fn take_state(self, states: &mut axum_starter::StateCollector) {
        let mongo = states
            .take::<MongoDatabaseOperate>()
            .expect("Using after set database");
        let manager = PushManager::new(mongo.clone());
        states.insert(mongo);

        let req_client = states
            .take::<RequestClient>()
            .expect("请在注册了 general_request-client 后");
        let client = ReqClient(req_client.clone());
        states.insert(req_client);

        let (mob_push, push_sender, mut error_chan) =
            MobPusher::new(client, manager, 16);

        tokio::spawn(mob_push.start_up());
        tokio::spawn(async move {
            while let Some(_t) = error_chan.recv().await {}
        });

        let sql = states
            .take::<SqlDatabaseOperate>()
            .expect("在注册了 Sql数据库后添加该事件");
        let manager = MobPushManage {
            push_sender,
            db: sql.clone(),
        };

        states.insert(manager);
        states.insert(sql);
    }
}

pub struct MobPushManage {
    push_sender: Sender<CookiePushEntity>,
    db: SqlDatabaseOperate,
}

impl MobPushManage {
    pub async fn pushing(
        &self,
        datasource: String,
        unique_key: String,
        image: Option<String>,
        content: String,
    ) -> Result<(), InternalError> {
        let info = self
            .db
            .fetcher()
            .datasource()
            .find_pushing_info_by_datasource_id_and_db_unique_key(
                &datasource,
                &unique_key,
            )
            .await?;

        let entity = CookiePushEntity::new(info, image, content);
        self.push_sender
            .send(entity)
            .await
            .map_err(|_| InternalError::PusherDeath)?;
        Ok(())
    }
}

#[prepare(MobPush)]
pub fn start_mob_push(config: MobPushConfig) -> impl PrepareStateEffect {
    mob_push::set_config(config);
    InitManage
}
