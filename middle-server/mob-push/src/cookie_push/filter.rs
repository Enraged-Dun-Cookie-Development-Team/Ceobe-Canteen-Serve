use std::collections::HashSet;

use mob_push::{PushEntity, SubscribeFilter};
use uuid::Uuid;

use super::{error::InternalError, push_entity::CookiePushEntity};

pub struct CookieSubScribeFilter {
    subscribes: HashSet<Uuid>,
}

impl CookieSubScribeFilter {
    pub fn new(set: HashSet<Uuid>) -> Self {
        Self { subscribes: set }
    }
}

impl SubscribeFilter for CookieSubScribeFilter {
    type Data = CookiePushEntity;

    type Err = InternalError;

    fn filter(
        &self,
        input: impl Iterator<Item = Self::Data>,
    ) -> Result<Vec<Self::Data>, Self::Err> {
        Ok(input
            .filter(|data| self.subscribes.contains(data.get_resource()))
            .collect())
    }

    fn contains(
        &self,
        target: &<Self::Data as mob_push::PushEntity>::Resource,
    ) -> Result<bool, Self::Err> {
        Ok(self.subscribes.contains(target))
    }
}
