use std::borrow::Cow;

use serde::{ser::SerializeStruct, Serialize};

use crate::{
    push_notify::{
        android::AndroidNotify, ios::IosNotify, Notify, NotifySerialize,
        SerializeInformation,
    },
    PushEntity, PushForward,
};

pub struct Forward(PushForward);

impl Forward {
    pub fn new<T: PushEntity>(data: &T) -> Self {
        let mut push_forward = PushForward::HomePage;
        data.push_forward(&mut push_forward);

        Self(push_forward)
    }
}

impl Serialize for Forward {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer, {
        let mut s = serializer.serialize_struct("pushForward", self.0.serialize_field())?;
        NotifySerialize::serialize::<S>(&self.0, &mut s)?;
        s.end()
    }
}

pub struct PushNotify<'p, A = Notify<AndroidNotify>, I = Notify<IosNotify>>
    where A: Serialize + 'static,
          I: Serialize + 'static, {
    body: &'p str,
    title: Cow<'p, str>,
    offline_seconds: Option<u64>,
    android_notify: A,
    ios_notify: I,
}

impl<'p> PushNotify<'p> {
    pub fn new_with_builder<T: PushEntity>(data: &'p T) -> Self {
        let mut android_notify = AndroidNotify::default().into_notify();
        data.android_notify(&mut android_notify);
        let mut ios_notify = IosNotify::default().into_notify();
        data.ios_notify(&mut ios_notify);

        Self {
            body: data.get_send_content().as_ref(),
            title: data.get_title(),
            offline_seconds: data.expired_time().map(|duration| duration.as_secs()),
            android_notify,
            ios_notify,
        }
    }
}

impl<'p> Serialize for PushNotify<'p> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer, {
        let mut len = 4 + self.offline_seconds.is_some()as usize;


        if self.android_notify.need_serialize() {
            len += 1;
        }
        if self.ios_notify.need_serialize() {
            len += 1;
        }

        let mut notify = serializer.serialize_struct("PushNotify", len)?;

        notify.serialize_field("plats", &[1, 2])?;
        notify.serialize_field("content", &self.body)?;
        notify.serialize_field("type", &1)?;
        notify.serialize_field("title", &self.title)?;
        if self.offline_seconds.is_some() {
            notify.serialize_field("offline_seconds", &self.offline_seconds)?;
        }
        if self.android_notify.need_serialize() {
            notify.serialize_field("androidNotify", &self.android_notify)?;
        }
        if self.ios_notify.need_serialize() {
            notify.serialize_field("iosNotify", &self.ios_notify)?;
        }


        notify.end()
    }
}
