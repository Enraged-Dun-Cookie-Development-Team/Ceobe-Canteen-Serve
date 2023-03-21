use serde::ser::SerializeStruct;

use crate::push_notify::NotifySerialize;
#[derive(Debug, Clone)]
/// 表示有新的可用内容。
///  App 在后台启动了或恢复运行了，
/// `application:didReceiveRemoteNotification:fetchCompletionHandler` 被调用了
pub struct ContentAvailable;

impl NotifySerialize for ContentAvailable {
    fn serialize_field(&self) -> usize {
        1
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        struct_serialize.serialize_field("contentAvailable", &1)
    }
}
