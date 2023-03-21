use serde::{ser::SerializeStruct, Serialize};

use crate::push_notify::NotifySerialize;

#[derive(Debug, Clone)]
/// APNs的category字段，只有IOS8及以上系统才支持此参数推送
pub struct Category(pub String);

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl NotifySerialize for Category {
    fn serialize_field(&self) -> usize {
        1
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        struct_serialize.serialize_field("category", &self.0)
    }
}
#[derive(Debug, Clone)]
/// APNs通知，通过这个字段指定声音，
/// - 默认为default（系统默认声音）
/// - 如设置为空值则为静音。
/// - 如设置为其他字符，则需要您的应用中配置了该声音才可以正常发声。
pub enum IosPushSound {
    /// 默认声音
    Default,
    /// 静音
    None,
    /// 用户定义
    Custom(String),
}

impl Serialize for IosPushSound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            IosPushSound::Default => serializer.serialize_str("default"),
            IosPushSound::None => serializer.serialize_none(),
            IosPushSound::Custom(s) => serializer.serialize_str(s),
        }
    }
}

impl NotifySerialize for IosPushSound {
    fn serialize_field(&self) -> usize {
        1
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        struct_serialize.serialize_field("sound", self)
    }
}
