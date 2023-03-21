use std::collections::HashSet;

use ceobe_user::{ToCeobe, ToCeobeUser};

use mob_push::{SubscribeFilter, UserMobId};
use mongo_models::mongo_connection::MongoDatabaseOperate;

use super::{
    error::InternalError, filter::CookieSubScribeFilter,
    push_entity::CookiePushEntity,
};

pub struct PushManager {
    db: MongoDatabaseOperate,
}

impl PushManager {
    pub fn new(db: MongoDatabaseOperate) -> Self {
        Self { db }
    }
}

impl mob_push::UserSubscribeManage for PushManager {
    type UserIdentify = User;

    type PushData = CookiePushEntity;

    type Filter = CookieSubScribeFilter;

    type Err = InternalError;

    fn fetch_subscribe_filter<'life0, 'life1, 'async_trait>(
        &'life0 self,
        user_id: &'life1 Self::UserIdentify,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self::Filter, Self::Err>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let data = self
                .db
                .ceobe()
                .user()
                .property()
                .find_datasource_list_by_mob(
                    mongo_models::ceobe::user_property::models::UserMobId {
                        mob_id: user_id.0.clone(),
                    },
                )
                .await?
                .into_iter()
                .map(|uuid| {
                    let uuid = uuid.bytes();
                    uuid::Uuid::from_bytes(uuid)
                })
                .collect::<HashSet<_>>();
            Ok(CookieSubScribeFilter::new(data))
        })
    }

    fn check_subscribed<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        user_id: &'life1 Self::UserIdentify,
        data_resource: &'life2<Self::PushData as mob_push::PushEntity> ::Resource,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, Self::Err>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let filter = self.fetch_subscribe_filter(user_id).await?;
            filter.contains(data_resource)
        })
    }

    fn fetch_all_subscriber<'life0, 'life1, 'async_trait>(
        &'life0 self,
        data_resource: &'life1<Self::PushData as mob_push::PushEntity> ::Resource,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<Vec<Self::UserIdentify>, Self::Err>,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let uuid = data_resource.clone().into_bytes();
            let uuid =
                mongo_models::mongodb::bson::uuid::Uuid::from_bytes(uuid);
            let db = self
                .db
                .ceobe()
                .user()
                .property()
                .find_all_subscribed_user_mob_id(uuid)
                .await?
                .into_iter()
                .map(|user| User(user.mob_id))
                .collect();
            Ok(db)
        })
    }
}

pub struct User(String);

impl UserMobId for User {
    type MobId = String;

    fn get_mob_id(&self) -> Self::MobId {
        self.0.clone()
    }
}
