use serde::{Serialize, ser::SerializeStruct};

use crate::{
    PushEntity,
};

use super::{batch_user::BatchUsers, BATCH_SIZE, push_action::{PushNotify, Forward}};

pub(crate) struct BatchPush<'u, 's, 'p, 'key> {
    pub(crate) users: BatchUsers<'u, 's, BATCH_SIZE>,
    pub(crate) push_notify: PushNotify<'p>,
    pub(crate) push_forward: Forward,
    pub(crate) secret_key: &'key str,
}

impl<'u, 's, 'p, 'key> BatchPush<'u, 's, 'p, 'key> {
    pub(crate) fn new<E: PushEntity>(
        users: BatchUsers<'u,'s,BATCH_SIZE>, entity: &'p E, secret_key: &'key str,
    ) -> Self {
        Self {
            users,
            push_notify: PushNotify::new_with_builder(entity),
            push_forward: Forward::new(entity),
            secret_key,
        }
    }
}

impl<'u, 's, 'p, 'key> Serialize for BatchPush<'u, 's, 'p, 'key> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut push_body = serializer.serialize_struct("CreatePush", 4)?;

        push_body.serialize_field("source", &"webapi")?;
        push_body.serialize_field("appkey", self.secret_key)?;
        push_body.serialize_field("pushTarget", &self.users)?;
        push_body.serialize_field("pushNotify", &self.push_notify)?;
        push_body.serialize_field("pushForward", &self.push_forward)?;

        push_body.end()
    }
}
