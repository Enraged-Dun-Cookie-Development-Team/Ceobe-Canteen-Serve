use std::ops::BitAnd;

use serde::ser::SerializeStruct;

use crate::push_notify::NotifySerialize;
#[derive(Debug, Clone)]

/// 自定义声音
pub struct Sound(pub String);

impl From<&str> for Sound {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl NotifySerialize for Sound {
    fn serialize_field(&self) -> usize {
        1
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        struct_serialize.serialize_field("sound", &self.0)
    }
}
#[derive(Debug, Clone)]

/// 提示音类型
pub enum WarnSound {
    /// 1 提示音
    Prompt,
    /// 2 震动
    Vibration,
    /// 3 指示灯
    IndicatorLight,
}

impl From<WarnSound> for Warn {
    fn from(s: WarnSound) -> Self {
        Warn(vec![s])
    }
}

impl WarnSound {
    pub fn new_prompt() -> Self {
        Self::Prompt
    }
    pub fn new_vibration() -> Self {
        Self::Vibration
    }
    pub fn new_indicator_light() -> Self {
        Self::IndicatorLight
    }
}

impl WarnSound {
    fn to_code(&self) -> char {
        match self {
            WarnSound::Prompt => '1',
            WarnSound::Vibration => '2',
            WarnSound::IndicatorLight => '3',
        }
    }
}

#[derive(Debug, Clone)]

/// 提醒类型，可多选组合
pub struct Warn(pub Vec<WarnSound>);

impl NotifySerialize for Warn {
    fn serialize_field(&self) -> usize {
        1
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        let s = self.0.iter().map(WarnSound::to_code).collect::<String>();
        struct_serialize.serialize_field("warn", &s)
    }
}

impl BitAnd<WarnSound> for WarnSound {
    type Output = Warn;

    fn bitand(self, rhs: Self) -> Self::Output {
        Warn(vec![self, rhs])
    }
}

impl BitAnd<Warn> for WarnSound {
    type Output = Warn;

    fn bitand(self, mut rhs: Warn) -> Self::Output {
        rhs.0.push(self);
        rhs
    }
}

impl BitAnd<WarnSound> for Warn {
    type Output = Self;

    fn bitand(mut self, rhs: WarnSound) -> Self::Output {
        self.0.push(rhs);
        self
    }
}
